use uloop::{Epoll, ULoopFd, ULoopFlags};
use std::rc::Rc;
use std::cell::RefCell;
use std::time::{Instant};
use nix::unistd::Pid;
use uloop::signal::ULoopSignal;

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
                      a.source_mut::<ULoopSignal,_>(|fd|{
                          fd.handle();
                      });
                      dbg!(now.elapsed().as_secs());
                  })));

    poll.poll(None);
}
