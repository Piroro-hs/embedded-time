//! Abstraction for hardware timers/clocks

use crate::{
    duration::Duration, fixed_point::FixedPoint, fraction::Fraction, instant::Instant,
    time_int::TimeInt, timer::param, timer::Timer,
};
use core::{
    fmt::{self, Formatter},
    hash::Hash,
};

/// Potential `Clock` errors
#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Error {
    /// Exact cause of failure is unknown
    Unspecified,
    /// The clock has either stopped or never started
    NotRunning,
}

impl Default for Error {
    fn default() -> Self {
        Self::Unspecified
    }
}

/// The `Clock` trait provides an abstraction for hardware-specific timer peripherals, external
/// timer devices, RTCs, etc.
///
/// The `Clock` is characterized by an inner unsigned integer storage type (either [`u32`] or
/// [`u64`]), a [`u32`]/[`u32`] [`Fraction`] defining the duration (in seconds) of one
/// count of the `Clock`, and a custom error type representing errors that may be generated by the
/// implementation.
///
/// In addition to the [`Clock::try_now()`] method which returns an [`Instant`],
/// software [`Timer`]s can be spawned from a `Clock` object.
pub trait Clock: Sized {
    /// The type to hold the tick count
    type T: TimeInt + Hash;

    /// The duration of one clock tick in seconds, AKA the clock precision.
    const SCALING_FACTOR: Fraction;

    /// Get the current Instant
    ///
    /// # Errors
    ///
    /// - [`Error::NotRunning`]
    /// - [`Error::Unspecified`]
    fn try_now(&self) -> Result<Instant<Self>, Error>;

    /// Spawn a new, `OneShot` [`Timer`] from this clock
    fn new_timer<Dur: Duration>(
        &self,
        duration: Dur,
    ) -> Timer<param::OneShot, param::Armed, Self, Dur>
    where
        Dur: FixedPoint,
    {
        Timer::<param::None, param::None, Self, Dur>::new(&self, duration)
    }
}

/// A duration unit type for specific [`Clock`](clock/trait.Clock.html)
#[derive(Hash, Debug, Default)]
pub struct ClockDuration<Clock: crate::Clock>(pub Clock::T);

impl<Clock: crate::Clock> Clone for ClockDuration<Clock> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<Clock: crate::Clock> Copy for ClockDuration<Clock> {}

impl<Clock: crate::Clock> Duration for ClockDuration<Clock> {}

impl<Clock: crate::Clock> FixedPoint for ClockDuration<Clock> {
    type T = Clock::T;
    const SCALING_FACTOR: Fraction = Clock::SCALING_FACTOR;

    /// See [Constructing a duration](trait.Duration.html#constructing-a-duration)
    fn new(value: Self::T) -> Self {
        Self(value)
    }

    /// See [Get the integer part](trait.Duration.html#get-the-integer-part)
    fn integer(&self) -> &Self::T {
        &self.0
    }
}

impl<Clock: crate::Clock> fmt::Display for ClockDuration<Clock> {
    /// See [Formatting](trait.Duration.html#formatting)
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}
