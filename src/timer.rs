use nix::sys::time::{TimeSpec, TimeValLike};
use nix::sys::timerfd::{ClockId, Expiration, TimerFd, TimerFlags, TimerSetTimeFlags};
use std::time::{Instant,Duration};
use crate::event::event::EventSource;
use std::os::unix::io::RawFd;
use std::os::unix::io::AsRawFd;
pub struct ULoopTimer(TimerFd);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ULoopExpiration {
    OneShot(Duration),
    IntervalDelayed(Duration, Duration),
    Interval(Duration),
}

impl From<ULoopExpiration> for Expiration {
    fn from(ut: ULoopExpiration) -> Expiration {
        match ut {
            ULoopExpiration::OneShot(t)=>{
                Expiration::OneShot(TimeSpec::milliseconds(t.as_millis() as i64))
            }
            ULoopExpiration::IntervalDelayed(delay,t)=>{
                Expiration::IntervalDelayed(TimeSpec::milliseconds(delay.as_millis() as i64),
                                            TimeSpec::milliseconds(t.as_millis() as i64))
            }
            ULoopExpiration::Interval(t)=>{
                Expiration::Interval(TimeSpec::milliseconds(t.as_millis() as i64))
            }
        }

    }
}

impl EventSource for ULoopTimer{
    fn get_raw_fd(&self)->RawFd{
        self.0.as_raw_fd()
    }
}

impl ULoopTimer{
    pub fn new()->Self{
        Self(TimerFd::new(ClockId::CLOCK_MONOTONIC, TimerFlags::empty()).unwrap())
    }

    pub fn set(&self, t:ULoopExpiration){
        self.0.set(
            t.into(),
            TimerSetTimeFlags::empty(),
        );
    }

    pub fn wait(&self){
        self.0.wait();
    }
}