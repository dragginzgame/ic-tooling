use std::io::{self, Read, Write};

pub(in crate::artifact) struct DeniedReader;

impl Read for DeniedReader {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        Err(io::ErrorKind::PermissionDenied.into())
    }
}

pub(in crate::artifact) struct InvalidReader;

impl Read for InvalidReader {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        Ok(buffer.len() + 1)
    }
}

pub(in crate::artifact) struct InvalidWriter;

impl Write for InvalidWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        Ok(bytes.len() + 1)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
