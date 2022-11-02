use core::fmt::{self, Write};
use core::ops::Add;

/// Music interval in semitones.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Interval {
    semitones: u8,
}

impl Interval {
    pub const UNISON: Self = Self::new(0);

    pub const MINOR_SECOND: Self = Self::new(1);

    pub const MAJOR_SECOND: Self = Self::new(2);

    pub const MINOR_THIRD: Self = Self::new(3);

    pub const MAJOR_THIRD: Self = Self::new(4);

    pub const PERFECT_FOURTH: Self = Self::new(5);

    pub const TRITONE: Self = Self::new(6);

    pub const PERFECT_FIFTH: Self = Self::new(7);

    pub const MINOR_SIXTH: Self = Self::new(8);
    pub const MAJOR_SIXTH: Self = Self::new(9);

    pub const MINOR_SEVENTH: Self = Self::new(10);
    pub const MAJOR_SEVENTH: Self = Self::new(11);

    pub const THIRTEENTH: Self = Self::new(21);

    pub const fn new(semitones: u8) -> Self {
        Self { semitones }
    }

    pub const fn semitones(self) -> u8 {
        self.semitones
    }
}

impl From<u8> for Interval {
    fn from(semitones: u8) -> Self {
        Self::new(semitones)
    }
}

impl From<Interval> for u8 {
    fn from(interval: Interval) -> Self {
        interval.semitones()
    }
}

impl Add for Interval {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.semitones() + rhs.semitones())
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Interval::UNISON => f.write_char('1'),
            Interval::MAJOR_THIRD => f.write_char('3'),
            Interval::PERFECT_FIFTH => f.write_char('5'),
            Interval::MINOR_SEVENTH => f.write_str("m7"),
            _ => todo!(),
        }
    }
}
