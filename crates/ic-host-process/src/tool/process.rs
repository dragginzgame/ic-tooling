use super::{
    CompletedExecution, ExecutionContext, ExecutionError, ExecutionEvidence, ExecutionFailure,
    ExecutionOperation, OutputLimit, OutputLimits, OutputStream, SuccessfulExit,
};
use crate::child::{CleanupPolicy, OwnedChild};
use ic_host_artifacts::artifact::buffer::reserve_bounded;
use rustix::fs::{OFlags, fcntl_getfl, fcntl_setfl};
use std::{
    ffi::OsString,
    io::{self, BufReader, Read, Write},
    os::fd::AsFd,
    panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
    path::Path,
    process::{ChildStdout, Command, ExitStatus, Stdio},
    time::{Duration, Instant},
};

#[cfg(test)]
mod tests;

const POLL_INTERVAL: Duration = Duration::from_millis(2);

enum Exchange<'a> {
    Capture,
    Communicate {
        input: Option<&'a [u8]>,
        successful_exit: SuccessfulExit,
    },
    Drain,
}

impl<'a> Exchange<'a> {
    const fn parameters(self) -> (bool, Option<&'a [u8]>, SuccessfulExit) {
        match self {
            Self::Capture => (false, None, SuccessfulExit::Cleanup),
            Self::Communicate {
                input,
                successful_exit,
            } => (false, input, successful_exit),
            Self::Drain => (true, None, SuccessfulExit::Cleanup),
        }
    }
}

#[derive(Clone, Copy)]
pub(super) enum CleanupScope {
    DirectChild,
    ProcessGroup {
        cleanup: CleanupPolicy,
        deadline: Option<Instant>,
    },
}

pub(super) fn capture(
    path: &Path,
    arguments: &[OsString],
    context: &ExecutionContext<'_>,
    limits: OutputLimits,
) -> Result<CompletedExecution, ExecutionError> {
    let mut command = Command::new(path);
    command
        .args(arguments)
        .current_dir(context.current_dir)
        .env_clear()
        .envs(context.environment.iter().map(|(key, value)| (key, value)));
    capture_command(
        &mut command,
        limits,
        CleanupScope::DirectChild,
        true,
        || false,
    )
}

pub(super) fn capture_command(
    command: &mut Command,
    mut limits: OutputLimits,
    scope: CleanupScope,
    capture_output: bool,
    mut cancelled: impl FnMut() -> bool,
) -> Result<CompletedExecution, ExecutionError> {
    let started = Instant::now();
    let deadline = match scope {
        CleanupScope::DirectChild => None,
        CleanupScope::ProcessGroup { deadline, .. } => deadline,
    };
    let failure = if cancelled() {
        Some(ExecutionFailure::Cancelled)
    } else if deadline.is_some_and(|deadline| deadline <= Instant::now()) {
        Some(ExecutionFailure::TimedOut)
    } else {
        None
    };
    if let Some(failure) = failure {
        return Err(ExecutionError {
            failure,
            evidence: ExecutionEvidence::default(),
            cleanup: None,
        });
    }
    if let Some(deadline) = deadline {
        let remaining = deadline.saturating_duration_since(started);
        limits.timeout = Some(
            limits
                .timeout
                .map_or(remaining, |timeout| timeout.min(remaining)),
        );
    }
    if capture_output {
        command
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
    }
    let spawned = match scope {
        CleanupScope::DirectChild => OwnedChild::spawn_direct(command),
        CleanupScope::ProcessGroup { cleanup, .. } => OwnedChild::spawn(command, cleanup),
    };
    let mut child = spawned.map_err(|source| ExecutionError {
        failure: io_failure(ExecutionOperation::Spawn, source),
        evidence: ExecutionEvidence::default(),
        cleanup: None,
    })?;
    let stdout = child.take_stdout().map(BufReader::new);
    exchange(
        &mut child,
        stdout,
        Exchange::Capture,
        limits,
        started,
        cancelled,
        |_, _| Ok(()),
    )
}

pub(super) fn communicate(
    child: &mut OwnedChild,
    input: Option<&[u8]>,
    limits: OutputLimits,
    successful_exit: SuccessfulExit,
    cancelled: impl FnMut() -> bool,
    output: impl FnMut(OutputStream, &[u8]) -> io::Result<()>,
) -> Result<CompletedExecution, ExecutionError> {
    let stdout = child.take_stdout().map(BufReader::new);
    exchange(
        child,
        stdout,
        Exchange::Communicate {
            input,
            successful_exit,
        },
        limits,
        Instant::now(),
        cancelled,
        output,
    )
}

pub(super) fn drain_output(
    child: &mut OwnedChild,
    stdout: Option<BufReader<ChildStdout>>,
    limits: OutputLimits,
    cancelled: impl FnMut() -> bool,
) -> Result<CompletedExecution, ExecutionError> {
    let stdout = stdout.or_else(|| child.take_stdout().map(BufReader::new));
    exchange(
        child,
        stdout,
        Exchange::Drain,
        limits,
        Instant::now(),
        cancelled,
        |_, _| Ok(()),
    )
}

fn exchange(
    child: &mut OwnedChild,
    stdout: Option<BufReader<ChildStdout>>,
    operation: Exchange<'_>,
    limits: OutputLimits,
    started: Instant,
    mut cancelled: impl FnMut() -> bool,
    mut output: impl FnMut(OutputStream, &[u8]) -> io::Result<()>,
) -> Result<CompletedExecution, ExecutionError> {
    let (allow_reaped, input, successful_exit) = operation.parameters();
    let timeout = limits.timeout;
    let mut status = None;
    let mut stdin = child.take_stdin();
    let mut stdout = CapturedStream::new(stdout, limits.stdout, OutputStream::Stdout);
    let mut stderr = CapturedStream::new(child.take_stderr(), limits.stderr, OutputStream::Stderr);
    // Resume the original panic after closing pipes and attempting cleanup.
    // The borrowed owner can outlive a caller's catch_unwind, so its Drop alone
    // cannot guarantee cleanup here. No callback is reused after unwinding.
    let result = catch_unwind(AssertUnwindSafe(|| {
        let admitted = child.is_owned()
            || (allow_reaped
                && child
                    .try_wait()
                    .map_err(|source| io_failure(ExecutionOperation::Wait, source))?
                    .is_some());
        if !admitted {
            return Err(io_failure(
                ExecutionOperation::Wait,
                io::ErrorKind::InvalidInput.into(),
            ));
        }
        if input.is_some() && stdin.is_none() {
            return Err(io_failure(
                ExecutionOperation::StdinPipe,
                io::ErrorKind::BrokenPipe.into(),
            ));
        }
        let mut remaining = input.unwrap_or_default();
        if remaining.is_empty() {
            drop(stdin.take());
        }
        stdin.as_ref().map(nonblocking).transpose()?;
        stdout
            .reader
            .as_ref()
            .map(|reader| nonblocking(reader.get_ref()))
            .transpose()?;
        stderr.reader.as_ref().map(nonblocking).transpose()?;
        loop {
            if cancelled() {
                return Err(ExecutionFailure::Cancelled);
            }
            if timeout.is_some_and(|timeout| started.elapsed() >= timeout) {
                return Err(ExecutionFailure::TimedOut);
            }
            let progress_in = if let Some(pipe) = &mut stdin {
                let progress = write_chunk(pipe, &mut remaining)?;
                if remaining.is_empty() {
                    drop(stdin.take());
                }
                progress
            } else {
                false
            };
            let progress_out = stdout.read_chunk(&mut output)?;
            let progress_err = stderr.read_chunk(&mut output)?;
            status = poll_completion(child, successful_exit)
                .map_err(|source| io_failure(ExecutionOperation::Wait, source))?;
            if let Some(status) = status
                && stdout.reader.is_none()
                && stderr.reader.is_none()
                && stdin.is_none()
            {
                if cancelled() {
                    return Err(ExecutionFailure::Cancelled);
                }
                // Retained success still needs timely caller admission. Ordinary
                // capture keeps its existing synchronous-cleanup exception.
                if successful_exit == SuccessfulExit::Retain
                    && timeout.is_some_and(|timeout| started.elapsed() >= timeout)
                {
                    return Err(ExecutionFailure::TimedOut);
                }
                return Ok(status);
            }
            if !progress_in && !progress_out && !progress_err {
                std::thread::sleep(timeout.map_or(POLL_INTERVAL, |timeout| {
                    POLL_INTERVAL.min(timeout.saturating_sub(started.elapsed()))
                }));
            }
        }
    }));
    // Close all pipes before cleanup. A descendant retaining a writer must not
    // cause a blocking drain or leave background reader threads after a timeout.
    drop(stdin);
    drop(stdout.reader.take());
    drop(stderr.reader.take());
    let evidence = ExecutionEvidence {
        status,
        stdout: stdout.bytes,
        stderr: stderr.bytes,
        stdout_truncated: stdout.truncated,
        stderr_truncated: stderr.truncated,
    };
    match result {
        Ok(result) => finish_capture(child, evidence, result),
        Err(panic) => {
            let _ = child.terminate();
            resume_unwind(panic)
        }
    }
}

fn poll_completion(
    child: &mut OwnedChild,
    successful_exit: SuccessfulExit,
) -> io::Result<Option<ExitStatus>> {
    if successful_exit == SuccessfulExit::Retain {
        child.poll_exit().and_then(|status| {
            if status.is_some_and(|status| !status.success()) {
                child.try_wait()
            } else {
                Ok(status)
            }
        })
    } else {
        child.try_wait()
    }
}

fn finish_capture(
    child: &mut OwnedChild,
    mut evidence: ExecutionEvidence,
    result: Result<ExitStatus, ExecutionFailure>,
) -> Result<CompletedExecution, ExecutionError> {
    match result {
        Ok(status) => Ok(CompletedExecution {
            status,
            stdout: evidence.stdout,
            stderr: evidence.stderr,
            stdout_truncated: evidence.stdout_truncated,
            stderr_truncated: evidence.stderr_truncated,
        }),
        Err(failure) => {
            let cleanup = match child.terminate() {
                Ok(status) => {
                    evidence.status = Some(status);
                    None
                }
                Err(error) => {
                    evidence.status = error.status;
                    Some(Box::new(error))
                }
            };
            Err(ExecutionError {
                failure,
                evidence,
                cleanup,
            })
        }
    }
}

fn write_chunk(writer: &mut impl Write, remaining: &mut &[u8]) -> Result<bool, ExecutionFailure> {
    let count = match writer.write(&remaining[..remaining.len().min(16 * 1024)]) {
        Ok(0) => {
            return Err(io_failure(
                ExecutionOperation::WriteInput,
                io::ErrorKind::WriteZero.into(),
            ));
        }
        Ok(count) => count,
        // Commands may deliberately stop reading and still supply useful status
        // and diagnostics. Closing early is not proof the input was consumed.
        Err(error) if error.kind() == io::ErrorKind::BrokenPipe => {
            *remaining = &[];
            return Ok(true);
        }
        Err(error)
            if matches!(
                error.kind(),
                io::ErrorKind::Interrupted | io::ErrorKind::WouldBlock
            ) =>
        {
            return Ok(false);
        }
        Err(source) => return Err(io_failure(ExecutionOperation::WriteInput, source)),
    };
    *remaining = &remaining[count..];
    Ok(true)
}

fn nonblocking(pipe: &impl AsFd) -> Result<(), ExecutionFailure> {
    let flags = fcntl_getfl(pipe)
        .map_err(|source| io_failure(ExecutionOperation::ReadPipeFlags, source.into()))?;
    fcntl_setfl(pipe, flags | OFlags::NONBLOCK)
        .map_err(|source| io_failure(ExecutionOperation::SetPipeFlags, source.into()))
}

struct CapturedStream<R> {
    reader: Option<R>,
    bytes: Vec<u8>,
    policy: OutputLimit,
    stream: OutputStream,
    truncated: bool,
}

impl<R: Read> CapturedStream<R> {
    const fn new(reader: Option<R>, policy: OutputLimit, stream: OutputStream) -> Self {
        Self {
            reader,
            bytes: Vec::new(),
            policy,
            stream,
            truncated: false,
        }
    }

    fn read_chunk(
        &mut self,
        output: &mut impl FnMut(OutputStream, &[u8]) -> io::Result<()>,
    ) -> Result<bool, ExecutionFailure> {
        let Some(pipe) = self.reader.as_mut() else {
            return Ok(false);
        };
        let mut buffer = [0; 16 * 1024];
        let limit = self.policy.retained_bytes();
        let terminate = matches!(self.policy, OutputLimit::Terminate(_));
        let allowance = if terminate {
            (limit - self.bytes.len())
                .saturating_add(1)
                .min(buffer.len())
        } else {
            buffer.len()
        };
        let count = match pipe.read(&mut buffer[..allowance]) {
            Ok(0) => {
                // A closed pipe is also the completion state; never read it again.
                drop(self.reader.take());
                return Ok(false);
            }
            Ok(count) => count,
            Err(source)
                if matches!(
                    source.kind(),
                    io::ErrorKind::Interrupted | io::ErrorKind::WouldBlock
                ) =>
            {
                return Ok(false);
            }
            Err(source) => return Err(io_failure(ExecutionOperation::ReadOutput, source)),
        };
        let stream = self.stream;
        let retained = reserve_bounded(&mut self.bytes, count, limit)
            .map_err(|source| ExecutionFailure::Allocation { stream, source })?;
        self.bytes.extend_from_slice(&buffer[..retained]);
        // Preserve observed overflow even when the sink rejects this chunk.
        self.truncated |= retained != count;
        let observed = if terminate { retained } else { count };
        if observed != 0 {
            output(stream, &buffer[..observed])
                .map_err(|source| ExecutionFailure::Observer { stream, source })?;
        }
        if retained != count && terminate {
            return Err(ExecutionFailure::OutputLimit { stream });
        }
        Ok(true)
    }
}

const fn io_failure(operation: ExecutionOperation, source: io::Error) -> ExecutionFailure {
    ExecutionFailure::Io { operation, source }
}
