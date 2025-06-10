use std::{error::Error, io::Write, os::fd::AsFd};

pub trait Encode {
    fn encode<C>(&self, encoder: C) -> Result<C::Ok, C::Error>
    where
        C: Encoder;
}

pub trait Encoder: Sized + Write {
    type Ok;
    type Error: Error;

    fn write_bytes(&mut self, bytes: &[u8]) -> Result<Self::Ok, Self::Error>;
    fn write_bytes_of_fd(
        &mut self,
        fd: impl AsFd,
        offset: usize,
        count: usize,
    ) -> Result<Self::Ok, Self::Error>;
    fn 
}
