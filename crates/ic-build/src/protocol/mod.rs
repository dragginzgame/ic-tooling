//! Bounded Cargo record decoding and exact candidate selection, with no execution.

use std::path::PathBuf;

use serde_json::Value;

use crate::Failure;

pub(crate) struct Decoder<'a> {
    package: &'a str,
    target: &'a str,
    record_limit: usize,
    candidate_limit: usize,
    candidates: usize,
    pending: Vec<u8>,
    candidate: Option<PathBuf>,
    finished: bool,
    pub(crate) refusal: Option<Failure>,
}

impl<'a> Decoder<'a> {
    pub(crate) fn new(
        package: &'a str,
        target: &'a str,
        record_limit: usize,
        candidate_limit: usize,
    ) -> Self {
        Self {
            package,
            target,
            record_limit,
            candidate_limit,
            candidates: 0,
            pending: Vec::new(),
            candidate: None,
            finished: false,
            refusal: None,
        }
    }

    pub(crate) fn feed(&mut self, bytes: &[u8]) {
        if self.refusal.is_some() {
            return;
        }
        for &byte in bytes {
            if byte == b'\n' {
                if let Err(error) = self.record() {
                    self.refusal = Some(error);
                    return;
                }
                self.pending.clear();
            } else {
                if self.pending.len() == self.record_limit {
                    self.refusal = Some(Failure::RecordLimit);
                    return;
                }
                if self.pending.try_reserve(1).is_err() {
                    self.refusal = Some(Failure::Allocation);
                    return;
                }
                self.pending.push(byte);
            }
        }
    }

    fn record(&mut self) -> Result<(), Failure> {
        if self.finished {
            return Err(Failure::Protocol("record after build completion"));
        }
        let value: Value = serde_json::from_slice(&self.pending).map_err(Failure::Json)?;
        let reason = string(&value, "reason")?;
        match reason {
            "build-finished" => {
                if value.get("success").and_then(Value::as_bool) != Some(true) {
                    return Err(Failure::Protocol("unsuccessful or invalid completion"));
                }
                self.finished = true;
            }
            "compiler-artifact" => {
                let package = string(&value, "package_id")?;
                let target = value
                    .get("target")
                    .ok_or(Failure::Protocol("missing target"))?;
                let name = string(target, "name")?;
                let kinds = strings(target, "kind")?;
                let crate_types = strings(target, "crate_types")?;
                let filenames = strings(&value, "filenames")?;
                let fresh = value
                    .get("fresh")
                    .and_then(Value::as_bool)
                    .ok_or(Failure::Protocol("missing fresh flag"))?;
                if package == self.package
                    && name == self.target
                    && kinds.contains(&"cdylib")
                    && crate_types.contains(&"cdylib")
                {
                    if fresh {
                        return Err(Failure::Protocol("fresh artifact in private target"));
                    }
                    for filename in filenames {
                        let path = PathBuf::from(filename);
                        if path
                            .extension()
                            .is_some_and(|extension| extension == "wasm")
                        {
                            self.candidates += 1;
                            if self.candidates > self.candidate_limit {
                                return Err(Failure::CandidateLimit);
                            }
                            if self.candidate.is_some() {
                                return Err(Failure::Protocol("ambiguous matching artifacts"));
                            }
                            self.candidate = Some(path);
                        }
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub(crate) fn finish(mut self) -> Result<PathBuf, Failure> {
        if let Some(error) = self.refusal.take() {
            return Err(error);
        }
        if !self.pending.is_empty() {
            return Err(Failure::Protocol("unterminated Cargo record"));
        }
        if !self.finished {
            return Err(Failure::Protocol("missing build completion"));
        }
        self.candidate.ok_or(Failure::Protocol("no exact artifact"))
    }
}

fn string<'a>(value: &'a Value, key: &str) -> Result<&'a str, Failure> {
    value
        .get(key)
        .and_then(Value::as_str)
        .ok_or(Failure::Protocol("missing or invalid string field"))
}

fn strings<'a>(value: &'a Value, key: &str) -> Result<Vec<&'a str>, Failure> {
    let array = value
        .get(key)
        .and_then(Value::as_array)
        .ok_or(Failure::Protocol("missing or invalid array field"))?;
    array
        .iter()
        .map(|item| {
            item.as_str()
                .ok_or(Failure::Protocol("invalid array entry"))
        })
        .collect()
}

#[cfg(test)]
mod tests;
