use uloop::{ULoopTimer, ULoopExpiration, Epoll,
             ULoopFd, ULoopFlags};
use std::rc::Rc;
use std::cell::RefCell;
use std::time::{Instant,Duration};

fn main() {
    let mut poll = Epoll::new().unwrap();

    let ut = Rc::new(RefCell::new(ULoopTimer::new()));

    ut.borrow().set(
        ULoopExpiration::Interval(Duration::from_millis(3000))
    );

    let ufd = Rc::new(RefCell::new(ULoopFd::new(Some(ut))));
    let now = Instant::now();
    poll.register(ufd.clone(),ULoopFlags::ULOOP_READ,
                  Some(Box::new(move |a,_b|{
                      let fd = a.fd.as_ref().unwrap().borrow();
                      let fd = fd.downcast_ref::<ULoopTimer>().unwrap();
                      fd.wait();
                      dbg!(now.elapsed().as_secs());
                  })));

    poll.poll(None);
}
