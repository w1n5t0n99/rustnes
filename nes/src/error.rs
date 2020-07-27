use ::nes_rom::RomError;
use std::io::Error;
use std::fmt;

#[derive(Debug)]
pub enum NesError {
    Custom(String),
    IO(Error),
    Rom(RomError),
    Mapper(String),
}

impl NesError {
    pub fn from_custom<M: Into<String>>(msg: M) -> NesError {
        NesError::Custom(msg.into())
    }

    pub fn from_mapper<M: Into<String>>(msg: M) -> NesError {
        NesError::Mapper(msg.into())
    }
}

impl fmt::Display for NesError {
    fn fmt( &self, fmt: &mut fmt::Formatter ) -> fmt::Result {
        match *self {
            NesError::Custom( ref message ) => { write!( fmt, "NES Error {}",  message ) },
            NesError::Mapper( ref message ) => { write!( fmt, "NES Error {}",  message ) },
            NesError::Rom( ref err) =>  { write!( fmt, "NES Error {}",  err ) },
            NesError::IO( ref err) => { write!( fmt, "NES Error {}",  err ) },
        }
    }
}

impl std::error::Error for NesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            NesError::Custom(_) => None,
            NesError::Mapper(_) => None,
            NesError::Rom (ref source) => Some(source),
            NesError::IO(_) => None,
        }
    }
}

impl From<std::io::Error> for NesError {
    fn from(err: std::io::Error) -> NesError {
        NesError::IO(err)
    }
}

impl From<RomError> for NesError {
    fn from(err: RomError) -> NesError {
        NesError::Rom(err)
    }
}