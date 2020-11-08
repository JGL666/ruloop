use crate::uloop::{ULoopFd};
use std::rc::Rc;
use std::cell::RefCell;

pub type ULoopEventCallback = Option<Box<dyn FnMut(&mut ULoopFd, i32)>>;
pub struct ULoopFdEvent{
    pub fd:Rc<RefCell<ULoopFd>>,
    pub callback:ULoopEventCallback
}

impl ULoopFdEvent{
    pub fn new(fd:Rc<RefCell<ULoopFd>>, callback:ULoopEventCallback)->Self{
        Self{
            fd,
            callback
        }
    }
}

#[derive(Default,Debug,Copy, Clone)]
pub struct Token(usize);

impl From<Token> for u64{
    fn from(t: Token) -> Self{
        t.to_u64()
    }
}

impl From<Token> for usize{
    fn from(t: Token) -> Self{
        t.to_usize()
    }
}

impl Token{
    pub fn new(n:usize)->Self{
        Token(n)
    }

    pub fn to_u64(&self)->u64{
        self.0 as u64
    }

    pub fn to_usize(&self)->usize{
        self.0 as usize
    }
}


pub struct EventList(Vec<Option<Box<ULoopFdEvent>>>);

impl EventList{
    pub fn new()->EventList{
        EventList(Vec::new())
    }


    pub fn append(&mut self, item:Box<ULoopFdEvent>)->Token{

        if let Some(index) = self.0.iter().position(Option::is_none){
            let t = Token::new(index);
            item.fd.borrow_mut().set_token(t);
            self.0[index] = Some(item);
            return t;
        }

        let t = Token::new(self.0.len());
        item.fd.borrow_mut().set_token(t);
        self.0.push(Some(item));
        t
    }

    pub fn get(&mut self, index:Token)->Option<&mut Box<ULoopFdEvent>>{
        if let Some(v) = self.0.get_mut::<usize>(index.into()){
            return v.as_mut()
        }
        None
    }
}
