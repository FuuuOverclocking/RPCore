use std::io;
use std::os::fd::AsRawFd;

use io_uring::types::Fd;
use io_uring::{cqueue, opcode, squeue, SubmissionQueue, Submitter};

use crate::defs::invocation_source::completion;
use crate::mpsc::RxWithEventFd;

impl<'a, Arg, Ret> completion::Proactor<Submitter<'a>, SubmissionQueue<'a>, cqueue::Entry, u64>
    for RxWithEventFd<Arg, Ret>
{
    type InitErr = io::Error;
    type SubmitErr = squeue::PushError;
    type OnCompleteErr = io::Error;

    fn init(&mut self, _submitter: &Submitter<'a>) -> Result<(), Self::InitErr> {
        // Optionally register eventfd and buffer with the kernel to improve performance.
        // Note: io_uring_register_eventfd is used to register for CQ completion
        // event notifications, and io_uring_register_files should be used instead.
        Ok(())
    }

    fn submit(&mut self, sq: &mut SubmissionQueue<'a>, token: u64) -> Result<(), Self::SubmitErr> {
        const BUF_LEN: usize = 8;
        static DONT_CARE: [u8; BUF_LEN] = [0; BUF_LEN];

        let efd = Fd(self.eventfd.as_raw_fd());
        let read_entry = opcode::Read::new(efd, &DONT_CARE as *const u8 as *mut u8, BUF_LEN as u32)
            .build()
            .user_data(token);

        // SAFETY: DONT_CARE is a statically valid buffer whose value we do not care about.
        // Although eventfd is not always valid, even if it is closed during io_uring
        // processing, it will only result in a negative return value (error) in the CQE,
        // and will not cause any memory safety issues.
        unsafe { sq.push(&read_entry) }
    }

    fn on_complete(&mut self, entry: cqueue::Entry) -> Result<(), Self::OnCompleteErr> {
        if entry.result() < 0 {
            Err(io::Error::from_raw_os_error(entry.result()))
        } else {
            Ok(())
        }
    }
}
