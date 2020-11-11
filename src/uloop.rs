use std::rc::Rc;
use std::os::raw::c_uchar;
use crate::event::event::EventSource;
use std::cell::RefCell;
use crate::event::list::Token;


#[derive(Default)]
pub struct ULoopFd{
    pub eof:bool,
    pub error:bool,
    pub registered:bool,
    pub flags:c_uchar,
    pub token:Option<Token>,
    pub fd:Option<Rc<RefCell<dyn EventSource>>>,
}


impl ULoopFd{
    pub fn new(fd:Option<Rc<RefCell<dyn EventSource>>>)->Self{
        Self{
            eof:false,
            error:false,
            registered:false,
            flags:0,
            token:None,
            fd
        }
    }

    pub fn set_token(&mut self, token:Token){
        self.token = Some(token);
    }

    pub fn source_ref<T:EventSource + 'static, F:FnMut(&T)>(&self, mut cb:F){
        let fd = self.fd.as_ref().unwrap().borrow();
        let fd = fd.downcast_ref::<T>().unwrap();
        cb(fd);
    }

    pub fn source_mut<T:EventSource + 'static,F:FnMut(&mut T)>(&mut self, mut cb:F){
        let mut fd = self.fd.as_ref().unwrap().borrow_mut();
        let fd = fd.downcast_mut::<T>().unwrap();
        cb(fd);
    }
}