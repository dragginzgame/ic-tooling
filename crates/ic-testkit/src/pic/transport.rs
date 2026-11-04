use crate::pocket_ic::ClientError;

/// Whether an error chain contains a typed `PocketIC` transport or body failure.
///
/// This is evidence for caller-owned recovery, not proof that an instance died
/// or permission to replay an update. Inspect [`ClientError::request_context`]
/// for accepted operation/ingress identities and uncertain execution. Canister
/// rejections, codec errors, server refusals and deadlines are not classified as
/// dead transports. Error messages are never parsed.
#[must_use]
pub fn is_dead_pocket_ic_transport_error(error: &(dyn std::error::Error + 'static)) -> bool {
    let mut current = Some(error);
    while let Some(candidate) = current {
        if let Some(error) = candidate.downcast_ref::<ClientError>() {
            return matches!(
                error,
                ClientError::Transport { .. } | ClientError::Body { .. }
            );
        }
        // io::Error::source skips the contained error itself.
        current = candidate
            .downcast_ref::<std::io::Error>()
            .and_then(std::io::Error::get_ref)
            .map(|inner| inner as &dyn std::error::Error)
            .or_else(|| candidate.source());
    }
    false
}

#[cfg(test)]
mod tests;
