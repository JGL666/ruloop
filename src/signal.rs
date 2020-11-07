use nix::sys::signalfd::SignalFd;
use nix::sys::signal::{self, SigmaskHow, raise, Signal, SigSet, sigprocmask};
use std::os::unix::io::RawFd;
use std::os::unix::io::AsRawFd;
use crate::event::event::EventSource;
use std::convert::TryFrom;
use nix::unistd::{Pid};
// #[derive(Ord, PartialOrd, Eq, PartialEq)]
pub struct ULoopSignalHandle{
    pid:Pid,
    callback:Option<Box<dyn FnMut()>>
}

pub struct ULoopSignal{
    signal_fd:SignalFd,
    list:Vec<Option<Box<ULoopSignalHandle>>>
}

impl ULoopSignalHandle{
    fn new(pid:Pid, callback:Option<Box<dyn FnMut()>>)->Self{
        Self{
            pid,
            callback
        }
    }
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
        self.parse_callback(Pid::this());
    }

    pub fn insert(&mut self, pid:Pid, callback:Option<Box<dyn FnMut()>>){
        let v = Box::new(ULoopSignalHandle::new(pid, callback));
        if let Some(index) = self.list.iter().position(Option::is_none){
            self.list[index] = Some(v);
        }else{
            self.list.push(Some(v));
        }
    }

    fn contain(&mut self, pid:Pid)->bool{
        match self.list.iter().position(|x|{
            match x.as_ref().map(|v|{
                v.pid == pid
            }){
                None=>{
                    false
                }
                Some(ret)=>{
                    ret
                }
            }
        }){
            None=>false,
            Some(_)=>true
        }

    }

    pub fn register(&mut self, pid:Pid, callback:Option<Box<dyn FnMut()>>){
        if self.contain(pid){
            return
        }

        self.insert(pid, callback);
    }

    pub fn unregister(){

    }

    fn parse_callback(&mut self, pid:Pid){
        let v = self.list.iter().position(|x|{
            if let Some(v) = x{
                 if v.pid == pid{
                     true
                 }    else{
                     false
                 }
            }else{
                false
            }

        });

        if let Some(id) = v{
            self.list[id].as_mut().map(|s|{
                if let Some(ref mut f) = s.callback{
                    f();
                }
            });
        }
    }
}