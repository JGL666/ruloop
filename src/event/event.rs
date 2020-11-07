use std::any::{TypeId};
use std::os::unix::io::RawFd;

pub trait EventSource{
    fn type_id(&self) -> TypeId
        where
            Self: 'static,
    {
        TypeId::of::<Self>()
    }

    fn get_raw_fd(&self)->RawFd;
}

impl dyn EventSource{
    pub fn is<T: EventSource + 'static>(&self) -> bool {
        let t = TypeId::of::<T>();
        let boxed = self.type_id();
        t == boxed
    }

    pub fn downcast_ref<T: EventSource + 'static>(&self) -> Option<&T> {
        if self.is::<T>() {
            unsafe { Some(&*(self as *const dyn EventSource as *const T)) }
        } else {
            None
        }
    }

    pub fn downcast_mut<T: EventSource + 'static>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            unsafe { Some(&mut *(self as *mut dyn EventSource as *mut T)) }
        } else {
            None
        }
    }
}