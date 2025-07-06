use std::os::fd::RawFd;

pub trait Recv {
    fn area(&mut self, size_hint: Option<usize>) -> &mut [u8];
    fn area_with_fds(&mut self, size_hint: Option<usize>) -> (&mut [u8], &mut [RawFd]);
}
