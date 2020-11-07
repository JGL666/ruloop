use nix::sys::signalfd::SignalFd;
use nix::sys::signal::{self, SigmaskHow, raise, Signal, SigSet, sigprocmask};
use std::os::unix::io::RawFd;
use std::os::unix::io::AsRawFd;
use crate::event::event::EventSource;
use std::convert::TryFrom;

pub struct ULoopSignalHandle{

}

pub struct ULoopSignal{
    signal_fd:SignalFd,
    list:Vec<Option<Box<ULoopSignalHandle>>>
}
impl EventSource for ULoopSignal{
    fn get_raw_fd(&self)->RawFd{
        self.signal_fd.as_raw_fd()
    }
}

impl ULoopSignal{
    pub fn new()->Self{
        let mut mask = SigSet::empty();
        println!("ssssssss");
        mask.add(signal::SIGUSR1);
        println!("ssssssss1");
        sigprocmask(SigmaskHow::SIG_BLOCK, Some(&mask), None);
        ULoopSignal{
            signal_fd: SignalFd::new(&mask).unwrap(),
            list:Vec::new()
        }
    }

    pub fn handle(&mut self){
        // And now catch that same signal.
        let res = self.signal_fd.read_signal().unwrap().unwrap();
        let signo = Signal::try_from(res.ssi_signo as i32).unwrap();
        dbg!(signo);
    }
}