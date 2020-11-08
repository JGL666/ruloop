use std::{io,os::unix::io::RawFd};
use std::rc::Rc;
use nix::sys::epoll::{self,EpollEvent,EpollOp,EpollFlags};
use nix::fcntl::{fcntl,FcntlArg,FdFlag};
use crate::uloop::{ULoopFd};
use crate::event::list::{Token,EventList,ULoopFdEvent,ULoopEventCallback};
use std::cell::RefCell;

#[derive(Debug)]
pub struct PollEvent{
    pub token:Token,
    pub event:ULoopFlags
}

bitflags! {
   pub struct ULoopFlags: u32 {
        const ULOOP_READ = (1 << 0);
        const ULOOP_WRITE = (1 << 1);
        const ULOOP_EDGE_TRIGGER = (1 << 2);
        const ULOOP_BLOCKING = (1 << 3);
        const ULOOP_EVENT_BUFFERED = (1 << 4);
        const ULOOP_ERROR_CB = (1 << 6);
        const ULOOP_EVENT_MASK = Self::ULOOP_READ.bits | Self::ULOOP_WRITE.bits;
        const ULOOP_EOF = (1 << 7);
        const ULOOP_ERROR = (1 << 8);
    }
}

impl From<EpollFlags> for ULoopFlags{
    fn from(e:EpollFlags)->Self{
        let mut ev = ULoopFlags::empty();
        if e.contains(EpollFlags::EPOLLRDHUP|EpollFlags::EPOLLIN|
            EpollFlags::EPOLLOUT|EpollFlags::EPOLLERR|EpollFlags::EPOLLHUP){
            ev |= ULoopFlags::ULOOP_ERROR
        }

        if e.contains(EpollFlags::EPOLLRDHUP){
            ev |= ULoopFlags::ULOOP_EOF
        }

        if e.contains(EpollFlags::EPOLLIN){
            ev |= ULoopFlags::ULOOP_READ
        }

        if e.contains(EpollFlags::EPOLLOUT){
            ev |= ULoopFlags::ULOOP_WRITE
        }

        ev
    }
}

fn no_nix_err(err: nix::Error) -> std::io::Error {
    match err {
        ::nix::Error::Sys(errno) => errno.into(),
        _ => unreachable!(),
    }
}
pub struct Epoll {
    poll_fd: RawFd,
    list:EventList
}

impl Epoll{
    pub fn new()->io::Result<Epoll>{
        let poll_fd = epoll::epoll_create().map_err(no_nix_err)?;
        let flag = fcntl(poll_fd, FcntlArg::F_GETFD).map_err(no_nix_err)?;
        let flag = FdFlag::from_bits(flag).unwrap();
        fcntl(poll_fd,
              FcntlArg::F_SETFD(flag | FdFlag::FD_CLOEXEC)).map_err(no_nix_err)?;

        Ok(Epoll{
            poll_fd,
            list:EventList::new()
        })
    }

    pub fn register(&mut self, fd:Rc<RefCell<ULoopFd>>, flags:ULoopFlags, callback:ULoopEventCallback){
        let mut events = EpollFlags::empty();
        let op = if fd.borrow().registered{
            EpollOp::EpollCtlMod
        }else{
            EpollOp::EpollCtlAdd
        };

        if flags.contains(ULoopFlags::ULOOP_READ){
            events = EpollFlags::EPOLLIN | EpollFlags::EPOLLRDHUP;
        }

        if flags.contains(ULoopFlags::ULOOP_WRITE){
            events = EpollFlags::EPOLLOUT;
        }

        if flags.contains(ULoopFlags::ULOOP_EDGE_TRIGGER){
            events = EpollFlags::EPOLLET;
        }

        let token = self.list.append(Box::new(ULoopFdEvent::new(
            fd.clone(),
            callback
        )));

        let mut ev = EpollEvent::new(events, token.into());
        dbg!(&ev);
        if let Some(f) = fd.borrow().fd.as_ref(){
            dbg!();
            let _ = epoll::epoll_ctl(self.poll_fd, op, f.borrow().get_raw_fd(), Some(&mut ev));

        }
    }

    pub fn del(&self, _:&mut ULoopFd){
        let _ = epoll::epoll_ctl(self.poll_fd, EpollOp::EpollCtlDel,
                         self.poll_fd, None);
    }

    pub fn poll(&mut self, timeout: Option<std::time::Duration>){

        let timeout = timeout.map(|t|t.as_millis() as isize).unwrap_or(-1);
        let mut events = [epoll::EpollEvent::empty(); 32];
        loop {
            let n_fds = epoll::epoll_wait(self.poll_fd, &mut events, timeout).unwrap();
            events.iter().take(n_fds).for_each(|item|{

                    self.list.get(Token::new(item.data() as usize))
                        .map(|f| {
                        if let Some(ref mut cb) = f.callback {
                            dbg!();
                            cb(&mut f.fd.borrow_mut(), 0);
                            dbg!();
                        }
                    });
                }
            );
        }
    }
}
