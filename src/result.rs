// (c) 2024 Pepijn Bakker
// This code is licensed under the AGPL-3.0 license (see LICENSE for details)

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}