use uloop::{ Epoll,ULoopStdIoIn,
            ULoopFd, ULoopFlags};
use std::rc::Rc;
use std::cell::RefCell;
use std::time::{Instant,Duration};

fn main() {
    let mut poll = Epoll::new().unwrap();

    let ut = Rc::new(RefCell::new(ULoopStdIoIn::new()));

    let ufd = Rc::new(RefCell::new(ULoopFd::new(Some(ut))));
    let now = Instant::now();
    poll.register(ufd.clone(),ULoopFlags::ULOOP_READ,
                  Some(Box::new(move |a,_b|{
                      let fd = a.fd.as_ref().unwrap().borrow();
                      let fd = fd.downcast_ref::<ULoopStdIoIn>().unwrap();
                      let mut s = String::new();
                      fd.0.read_line(&mut s);
                      dbg!(s);
                  })));

    poll.poll(None);
}
