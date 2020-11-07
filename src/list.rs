// use std::rc::Rc;
// use std::collections::BinaryHeap;
use crate::{ULoopFd, ULoopFdEvent};
// use crate::timer::ULoopTimerEvent;
//
pub struct EventList(Vec<Option<Box<ULoopFdEvent>>>);
// pub struct TimerList(BinaryHeap<Box<ULoopTimerEvent>>);
//
// impl TimerList{
//     pub fn new()->TimerList{
//         TimerList(BinaryHeap::new())
//     }
//
//     pub fn append(&mut self, item:Box<ULoopTimerEvent>){
//         self.0.push(item);
//     }
//
//     pub fn get(&mut self)->Option<&Box<ULoopTimerEvent>>{
//         // let v = self.0.get(index);
//          self.0.peek()
//     }
// }
//
impl EventList{
    pub fn new()->EventList{
        EventList(Vec::new())
    }


    pub fn append(&mut self, item:Box<ULoopFdEvent>){
        self.0.push(Some(item));
    }

    pub fn get(&mut self, index:usize)->Option<&mut Box<ULoopFdEvent>>{
        // let v = self.0.get(index);
        if let Some(v) = self.0.get_mut(index){
            return v.as_mut()
        }
        None
    }
}
