use std::rc::Rc;
use std::os::raw::c_uchar;
use crate::event::event::EventSource;
use std::cell::RefCell;

#[derive(Default)]
pub struct ULoopFd{
    pub eof:bool,
    pub error:bool,
    pub registered:bool,
    pub flags:c_uchar,
    pub fd:Option<Rc<RefCell<dyn EventSource>>>,
}

type ULoopEventCallback = Option<Box<dyn FnMut(&mut ULoopFd, i32)>>;
pub struct ULoopFdEvent{
    pub fd:Box<ULoopFd>,
    pub callback:ULoopEventCallback
}

impl ULoopFdEvent{
    pub fn new(fd:Box<ULoopFd>, callback:ULoopEventCallback)->Self{
        Self{
            fd,
            callback
        }
    }
}