mod event;
mod sys;
mod timer;
mod uloop;
mod list;
mod io;
mod signal;
use std::rc::Rc;
use event::event::EventSource;
use std::os::unix::io::RawFd;
use uloop::{ULoopFd,ULoopFdEvent};
use list::{EventList};
use io::stdio::ULoopStdIoIn;
use sys::epoll::{Epoll,ULoopFlags, Token};
// use timer::{ULoopTimerEvent, ULoopTimer};
use std::time::{Duration, Instant};
use crate::timer::{ULoopTimer,ULoopExpiration};
use crate::signal::{ULoopSignal};
use std::cell::RefCell;
use nix::sys::signal::{raise, SIGUSR1};
#[macro_use]
extern crate bitflags;



fn main() {
    let mut v = ULoopFdEvent::new(Box::new(ULoopFd::default()),
                              Some(Box::new(|a,b|{
                                  // let fd = a.fd.as_ref().unwrap().borrow().downcast_ref::<ULoopStdIoIn>().unwrap();
                                  let fd = a.fd.as_ref().unwrap().borrow();
                                  let fd = fd.downcast_ref::<ULoopStdIoIn>().unwrap();
                                  let mut s = String::new();
                                  fd.0.read_line(&mut s);
                                  dbg!(s);
                              })));
    v.fd.fd = Some(Rc::new(RefCell::new(ULoopStdIoIn::new())));
    let mut list = EventList::new();

    // let t = ULoopTimer::new();

    let poll = Epoll::new().unwrap();
    let mut v1 = ULoopFdEvent::new(Box::new(ULoopFd::default()),
                                  Some(Box::new(|a,b|{
                                      let fd = a.fd.as_ref().unwrap().borrow();
                                      let fd = fd.downcast_ref::<ULoopTimer>().unwrap();
                                      fd.wait();
                                      raise(SIGUSR1).expect("Error: raise(SIGUSR1) failed");
                                      dbg!();
                                  })));

    let ut = ULoopTimer::new();
    ut.set(ULoopExpiration::Interval(Duration::from_millis(3000)));
    v1.fd.fd = Some(Rc::new(RefCell::new(ut)));

    // let us = ULoopSignal::new();

    let mut v2 = ULoopFdEvent::new(Box::new(ULoopFd::default()),
                                   Some(Box::new(|a,b|{

                                       let mut fd = a.fd.as_ref().unwrap().borrow_mut();
                                       let fd = fd.downcast_mut::<ULoopSignal>().unwrap();
                                       println!("111111");
                                       fd.handle();
                                       println!("222222");
                                       dbg!();
                                   })));

    v2.fd.fd = Some(Rc::new(RefCell::new(ULoopSignal::new())));
    poll.register(&v.fd, ULoopFlags::ULOOP_READ, Token::new(0));
    poll.register(&v1.fd, ULoopFlags::ULOOP_READ, Token::new(1));
    poll.register(&v2.fd, ULoopFlags::ULOOP_READ, Token::new(2));
    list.append(Box::new(v));
    list.append(Box::new(v1));
    list.append(Box::new(v2));
    loop {
        // dbg!();
        let events = poll.poll(None);
        println!("{:?}", events);
        if let Ok(event) = events {
            for i in event {
                list.get(i.token.0).map(|f| {
                    if let Some(ref mut cb) = f.callback {
                        dbg!();
                        cb(&mut f.fd, 0);
                        dbg!();
                    }
                });
            }
        }
    }
}