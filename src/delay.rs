use crate::{clock::ClockDuration, fixed_point::FixedPoint, fraction::Fraction};
use core::ops::FnMut;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};

pub struct Delay<'a, Clock: crate::Clock, Fn: FnMut()> {
    clock: &'a Clock,
    wait: Fn,
}

impl<'a, Clock: crate::Clock, Fn: FnMut()> Delay<'a, Clock, Fn> {
    pub fn new(clock: &'a Clock, wait: Fn) -> Self {
        Self { clock, wait }
    }
}

impl<'a, Clock: crate::Clock, Fn: FnMut(), UXX: Into<Clock::T>> DelayMs<UXX>
    for Delay<'a, Clock, Fn>
{
    fn delay_ms(&mut self, ms: UXX) {
        let scaling_factor = Fraction::new(1, 1_000);
        let ms = ms.into();
        let tick = ClockDuration::<Clock>::convert_ticks(ms, scaling_factor).unwrap();
        let correction = if (ms
            * Clock::T::from(*ClockDuration::<Clock>::SCALING_FACTOR.denominator()))
            % Clock::T::from(*ClockDuration::<Clock>::SCALING_FACTOR.numerator())
            % 1_000.into()
            == 0.into()
        {
            1.into()
        } else {
            2.into()
        };
        let duration = ClockDuration::<Clock>::new(tick + correction);
        let timer = self.clock.new_timer(duration).start().unwrap();
        while !timer.is_expired().unwrap() {
            (self.wait)();
        }
    }
}

impl<'a, Clock: crate::Clock, Fn: FnMut(), UXX: Into<Clock::T>> DelayUs<UXX>
    for Delay<'a, Clock, Fn>
{
    fn delay_us(&mut self, us: UXX) {
        let scaling_factor = Fraction::new(1, 1_000_000);
        let us = us.into();
        let tick = ClockDuration::<Clock>::convert_ticks(us, scaling_factor).unwrap();
        let correction = if (us
            * Clock::T::from(*ClockDuration::<Clock>::SCALING_FACTOR.denominator()))
            % Clock::T::from(*ClockDuration::<Clock>::SCALING_FACTOR.numerator())
            % 1_000_000.into()
            == 0.into()
        {
            1.into()
        } else {
            2.into()
        };
        let duration = ClockDuration::<Clock>::new(tick + correction);
        let timer = self.clock.new_timer(duration).start().unwrap();
        while !timer.is_expired().unwrap() {
            (self.wait)();
        }
    }
}
