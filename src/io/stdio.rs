use std::io::Stdin;
use crate::event::event::EventSource;
use std::os::unix::io::RawFd;

pub struct ULoopStdIoIn(pub Stdin);
impl ULoopStdIoIn{
    pub fn new()->Self{
        ULoopStdIoIn(std::io::stdin())
    }
}
impl EventSource for ULoopStdIoIn{

    fn get_raw_fd(&self)->RawFd{
        0 as RawFd
    }
}