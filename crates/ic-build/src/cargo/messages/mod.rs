//! Shared Cargo message decoding and exact package/target matching.
use super::observation::schema;
use ic_host_process::framing::{FrameReadError, LineFramer};
use std::io;

/// Record terminal evidence independently of each caller's process/error policy.
#[derive(Default)]
pub(super) struct BuildCompletion(Option<bool>);

impl BuildCompletion {
    pub(super) fn record(&mut self, success: bool) -> bool {
        self.0 = Some(self.0.is_none() && success);
        self.0 == Some(true)
    }

    pub(super) fn require_success(&self) -> io::Result<()> {
        match self.0 {
            Some(true) => Ok(()),
            Some(false) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid Cargo build completion",
            )),
            None => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Cargo omitted successful build completion",
            )),
        }
    }
}

pub fn artifact_matches(
    artifact: &schema::Artifact,
    package: &str,
    name: &str,
    kind: &schema::TargetKind,
) -> bool {
    artifact.package_id.repr == package
        && artifact.target.name == name
        && artifact.target.kind.contains(kind)
}
pub struct MessageDecoder {
    framing: LineFramer,
    allow_text: bool,
}
impl MessageDecoder {
    pub const fn new(maximum: usize, allow_text: bool) -> Self {
        Self {
            framing: LineFramer::new(maximum as u64),
            allow_text,
        }
    }
    pub fn feed(
        &mut self,
        bytes: &[u8],
        mut emit: impl FnMut(schema::Message) -> io::Result<()>,
    ) -> io::Result<()> {
        let allow_text = self.allow_text;
        self.framing
            .feed(bytes, |frame| emit(Self::decode(frame, allow_text)?))
            .map_err(Self::framing_error)
    }
    pub fn finish(self, mut emit: impl FnMut(schema::Message) -> io::Result<()>) -> io::Result<()> {
        match self.framing.finish() {
            Ok(()) => Ok(()),
            // Cargo permits a final record without a newline.
            Err(FrameReadError::Unterminated(tail)) => emit(Self::decode(&tail, self.allow_text)?),
            Err(error) => Err(Self::framing_error(error)),
        }
    }
    fn framing_error(error: FrameReadError) -> io::Error {
        match error {
            FrameReadError::Io(source) => source,
            FrameReadError::TooLarge(_) => io::Error::new(
                io::ErrorKind::InvalidData,
                "Cargo JSON message exceeds caller allowance",
            ),
            other => io::Error::new(io::ErrorKind::InvalidData, other),
        }
    }
    fn decode(bytes: &[u8], allow_text: bool) -> io::Result<schema::Message> {
        match serde_json::from_slice(bytes) {
            Ok(message) => Ok(message),
            Err(_)
                if allow_text
                    && bytes
                        .iter()
                        .copied()
                        .find(|byte| !byte.is_ascii_whitespace())
                        != Some(b'{') =>
            {
                Ok(schema::Message::TextLine(
                    String::from_utf8_lossy(bytes).into_owned(),
                ))
            }
            Err(error) => Err(io::Error::new(io::ErrorKind::InvalidData, error)),
        }
    }
}

#[cfg(test)]
mod tests;
