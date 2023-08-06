use std::{fs, io};

/// Wrapper over IO streams: `io::Stdout`, `fs::File`.
pub enum IoWrap {
    Stdout(io::Stdout),
    File(fs::File),
}

impl io::Write for IoWrap {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Self::Stdout(stdout) => stdout.write(buf),
            Self::File(file) => file.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Self::Stdout(stdout) => stdout.flush(),
            Self::File(file) => file.flush(),
        }
    }
}
