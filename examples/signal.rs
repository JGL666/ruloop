use uloop::{ULoopSignal, Epoll,
            ULoopFd, ULoopFlags};
use std::rc::Rc;
use std::cell::RefCell;
use std::time::{Instant,Duration};
use nix::unistd::Pid;

fn main() {
    let mut poll = Epoll::new().unwrap();

    let ut = Rc::new(RefCell::new(ULoopSignal::new()));
    ut.borrow_mut().insert(Pid::this(), Some(Box::new(||{
        println!("hello {}",Pid::this());
    })));

    let ufd = Rc::new(RefCell::new(ULoopFd::new(Some(ut))));
    let now = Instant::now();
    poll.register(ufd.clone(),ULoopFlags::ULOOP_READ,
                  Some(Box::new(move |a,_b|{
                      let mut fd = a.fd.as_ref().unwrap().borrow_mut();
                      let fd = fd.downcast_mut::<ULoopSignal>().unwrap();
                      fd.handle();
                      dbg!();
                  })));

    poll.poll(None);
}
