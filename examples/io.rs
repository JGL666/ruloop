use uloop::{ Epoll,
            ULoopFd, ULoopFlags};
use std::rc::Rc;
use std::cell::RefCell;
use std::time::{Instant};
use uloop::io::stdio::ULoopStdIoIn;

fn main() {
    let mut poll = Epoll::new().unwrap();

    let ut = Rc::new(RefCell::new(ULoopStdIoIn::new()));

    let ufd = Rc::new(RefCell::new(ULoopFd::new(Some(ut))));
    let now = Instant::now();
    poll.register(ufd.clone(),ULoopFlags::ULOOP_READ,
                  Some(Box::new(move |a,_b|{
                      a.source_ref::<ULoopStdIoIn,_>(|fd|{
                          let mut s = String::new();
                          let _ = fd.0.read_line(&mut s);
                          dbg!(s);
                          dbg!(now.elapsed().as_secs());
                      });
                  })));

    poll.poll(None);
}
