use crate::{
    midi::{MidiNote, Octave},
    set::IntervalSet,
    Interval, Natural, Note, Pitch,
};
use core::{
    fmt::{self, Write},
    str::FromStr,
};

mod iter;
pub use self::iter::{Chords, Intervals, Iter};

/*
/// ```
/// use staff::{chord, midi, Pitch, Chord};
///
/// let notes = [midi!(C, 4),midi!(E, 4), midi!(G, 4)];
/// let chords = chord::chords(&notes);
///
/// let names = chords.map(|chord| chord.to_string());
/// assert!(names.eq(["C", "Em/C(no5)", "Gm/C"]));
/// ```
*/
pub fn chords<T>(midi_notes: T) -> Chords<T>
where
    T: AsRef<[MidiNote]>,
{
    Chords::new(midi_notes)
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Chord {
    pub root: MidiNote,
    pub bass: Option<MidiNote>,
    pub is_inversion: bool,
    pub intervals: IntervalSet,
}

impl Chord {
    pub fn new(root: MidiNote) -> Self {
        Self {
            root,
            bass: None,
            is_inversion: false,
            intervals: IntervalSet::default(),
        }
    }

    pub fn bass(mut self, bass_note: MidiNote) -> Self {
        self.bass = Some(bass_note);
        self
    }

    pub fn inversion(mut self, bass_note: MidiNote) -> Self {
        self.is_inversion = true;
        self.bass(bass_note)
    }

    pub fn interval(mut self, interval: Interval) -> Self {
        self.intervals.push(interval);
        self
    }

    pub fn root(self) -> Self {
        self.interval(Interval::UNISON)
    }

    /// ```
    /// use staff::{Chord, midi};
    ///
    /// let chord = Chord::major(midi!(C, 4))
    ///     .major_seventh()
    ///     .major_ninth();
    ///
    /// let midi_notes = [
    ///     midi!(C, 4),
    ///     midi!(E, 4),
    ///     midi!(G, 4),
    ///     midi!(B, 4),
    ///     midi!(D, 5),
    /// ];
    ///
    /// assert!(chord.into_iter().eq(midi_notes));
    /// ```
    pub fn major(root: MidiNote) -> Self {
        Self::new(root)
            .root()
            .interval(Interval::MAJOR_THIRD)
            .interval(Interval::PERFECT_FIFTH)
    }

    pub fn minor(root: MidiNote) -> Self {
        Self::new(root)
            .root()
            .interval(Interval::MINOR_THIRD)
            .interval(Interval::PERFECT_FIFTH)
    }

    pub fn seventh(root: MidiNote) -> Self {
        Self::major(root).interval(Interval::MINOR_SEVENTH)
    }

    pub fn major_seventh(self) -> Self {
        self.interval(Interval::MAJOR_SEVENTH)
    }

    pub fn minor_seventh(root: MidiNote) -> Self {
        Self::minor(root).interval(Interval::MINOR_SEVENTH)
    }

    pub fn major_ninth(self) -> Self {
        self.interval(Interval::MAJOR_NINTH)
    }

    pub fn half_diminished(root: MidiNote) -> Self {
        Self::new(root)
            .root()
            .interval(Interval::MINOR_THIRD)
            .interval(Interval::TRITONE)
            .interval(Interval::MINOR_SEVENTH)
    }

    /// ```
    /// use staff::{midi, Chord, Pitch};
    ///
    /// let notes = [midi!(E, 3), midi!(G, 3), midi!(C, 4)];
    /// let chord = Chord::from_midi(midi!(C, 4), notes).unwrap();
    ///
    /// assert_eq!(chord.to_string(), "C4/E3");
    ///
    /// assert!(chord.into_iter().eq(notes));
    /// ```
    pub fn from_midi<I>(root: MidiNote, iter: I) -> Option<Self>
    where
        I: IntoIterator<Item = MidiNote>,
    {
        let mut iter = iter.into_iter();
        let mut intervals = IntervalSet::default();
        let mut is_inversion = false;

        let bass_note = iter.next()?;

        let bass = if bass_note != root {
            is_inversion = true;
            Some(bass_note)
        } else {
            None
        };
        intervals.push(Interval::UNISON);

        let lowest_note = bass.unwrap_or(root);
        intervals.extend(iter.map(|midi| midi - lowest_note));

        for i in intervals.clone().into_iter() {
            dbg!(i);
        }

        Some(Self {
            root,
            bass,
            is_inversion,
            intervals,
        })
    }

    /// ```
    /// use staff::{midi, Chord};
    ///
    /// let chord = Chord::major(midi!(C, 4));
    /// ```
    pub fn intervals(self) -> IntervalSet {
        self.intervals
            .map(|interval| {
                let midi_note = self.bass.unwrap_or(self.root) + interval;
                dbg!(midi_note);
                midi_note.abs_diff(self.root)
            })
            .collect()
    }

    pub fn midi_notes(self) -> MidiNotes {
        MidiNotes {
            root: self.bass.unwrap_or(self.root),
            intervals: self.intervals,
        }
    }
}

pub struct MidiNotes {
    root: MidiNote,
    intervals: IntervalSet,
}

impl Iterator for MidiNotes {
    type Item = MidiNote;

    fn next(&mut self) -> Option<Self::Item> {
        self.intervals.next().map(|interval| self.root + interval)
    }
}

impl FromIterator<MidiNote> for Chord {
    fn from_iter<T: IntoIterator<Item = MidiNote>>(iter: T) -> Self {
        let mut notes = iter.into_iter();
        let root = notes.next().unwrap_or(MidiNote::from_byte(0));

        Self::from_midi(root, core::iter::once(root).chain(notes)).unwrap()
    }
}

impl IntoIterator for Chord {
    type Item = MidiNote;

    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            root: dbg!(self.bass.unwrap_or(self.root)),
            intervals: self.intervals,
        }
    }
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.root.fmt(f)?;

        dbg!("{:?}", self.clone().intervals().collect::<Vec<_>>());
        dbg!(
            "{:?}",
            self.clone().intervals().contains(Interval::PERFECT_FOURTH)
        );

        if self.clone().intervals().contains(Interval::MINOR_THIRD) {
            f.write_char('m')?
        } else if self.clone().intervals().contains(Interval::MAJOR_SECOND) {
            f.write_str("sus2")?
        } else if self.clone().intervals().contains(Interval::PERFECT_FOURTH) {
            f.write_str("sus4")?
        }

        let mut has_fifth = true;
        if self.clone().intervals().contains(Interval::TRITONE) {
            f.write_str("b5")?
        } else if !self.clone().intervals().contains(Interval::PERFECT_FIFTH) {
            has_fifth = false;
        }

        if self.clone().intervals().contains(Interval::MINOR_SEVENTH) {
            f.write_char('7')?
        } else if self.clone().intervals().contains(Interval::MAJOR_SEVENTH) {
            f.write_str("maj7")?
        }

        if let Some(bass) = self.bass {
            write!(f, "/{}", bass)?;
        }

        if !self.clone().intervals().contains(Interval::UNISON) {
            f.write_str("(no root)")?
        }

        if !has_fifth {
            f.write_str("(no5)")?
        }

        Ok(())
    }
}

impl FromStr for Chord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let natural: Natural = chars.next().unwrap().try_into().unwrap();

        let mut next = chars.next();
        let root: Pitch = match next {
            Some('b') => {
                next = chars.next();
                if next == Some('b') {
                    next = chars.next();
                    Note::double_flat(natural).into()
                } else {
                    Note::flat(natural).into()
                }
            }
            Some('#') => {
                next = chars.next();
                if next == Some('#') {
                    next = chars.next();
                    Note::double_sharp(natural).into()
                } else {
                    Note::sharp(natural).into()
                }
            }
            _ => natural.into(),
        };

        let mut chord = match next {
            Some('m') => {
                next = chars.next();
                Chord::minor(MidiNote::new(root, Octave::FOUR))
            }
            _ => Chord::major(MidiNote::new(root, Octave::FOUR)),
        };

        loop {
            if let Some(c) = next {
                match c {
                    'b' => match chars.next() {
                        Some(c) => match c {
                            '5' => chord.intervals.push(Interval::TRITONE),
                            _ => todo!(),
                        },
                        None => break,
                    },
                    '7' => chord.intervals.push(Interval::MINOR_SEVENTH),
                    _ => todo!(),
                }
                next = chars.next();
            } else {
                break;
            }
        }

        Ok(chord)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        midi::{self, MidiNote, Octave},
        Chord, Pitch,
    };

    #[test]
    fn it_parses_d_double_sharp_major() {
        let chord: Chord = "D##".parse().unwrap();
        assert_eq!(chord, Chord::major(MidiNote::new(Pitch::E, Octave::FOUR)));
    }

    #[test]
    fn it_parses_c_minor_seven() {
        let chord: Chord = "Cm7".parse().unwrap();
        assert_eq!(
            chord,
            Chord::minor_seventh(MidiNote::new(Pitch::C, Octave::FOUR))
        );
    }

    #[test]
    fn f() {
        let chord = Chord::from_midi(
            MidiNote::new(Pitch::C, Octave::FOUR),
            [
                MidiNote::new(Pitch::C, Octave::FOUR),
                MidiNote::new(Pitch::E, Octave::FOUR),
                MidiNote::new(Pitch::G, Octave::FOUR),
                MidiNote::new(Pitch::B, Octave::FOUR),
                MidiNote::new(Pitch::D, Octave::FIVE),
            ],
        )
        .unwrap();

        dbg!(chord.to_string());
    }
}
