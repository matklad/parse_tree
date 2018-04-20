use serde::{Serialize, Serializer, Deserialize, Deserializer};
use std::fmt;
use std::ops;

/// An text position in a source file
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextUnit(u32);

impl TextUnit {
    pub fn new(val: u32) -> TextUnit {
        TextUnit(val)
    }
}

impl Default for TextUnit {
    fn default() -> Self {
        TextUnit(0)
    }
}

impl fmt::Debug for TextUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl fmt::Display for TextUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<TextUnit> for u32 {
    fn from(tu: TextUnit) -> u32 {
        tu.0
    }
}

impl From<u32> for TextUnit {
    fn from(tu: u32) -> TextUnit {
        TextUnit::new(tu)
    }
}

impl ops::Add<TextUnit> for TextUnit {
    type Output = TextUnit;
    fn add(self, rhs: TextUnit) -> TextUnit {
        TextUnit(self.0 + rhs.0)
    }
}

impl ops::AddAssign<TextUnit> for TextUnit {
    fn add_assign(&mut self, rhs: TextUnit) {
        self.0 += rhs.0
    }
}

impl ops::Sub<TextUnit> for TextUnit {
    type Output = TextUnit;
    fn sub(self, rhs: TextUnit) -> TextUnit {
        TextUnit(self.0 - rhs.0)
    }
}

impl ops::SubAssign<TextUnit> for TextUnit {
    fn sub_assign(&mut self, rhs: TextUnit) {
        self.0 -= rhs.0
    }
}

/// A range of text in a source file
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TextRange {
    start: TextUnit,
    end: TextUnit,
}

impl fmt::Debug for TextRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl fmt::Display for TextRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}; {})", self.start(), self.end())
    }
}

impl TextRange {
    /// An length-0 range of text
    pub fn empty() -> TextRange {
        TextRange::from_to(TextUnit::new(0), TextUnit::new(0))
    }

    /// The left-inclusive range (`[from..to)`) between to points in the text
    pub fn from_to(from: TextUnit, to: TextUnit) -> TextRange {
        assert!(from <= to, "Invalid text range [{}; {})", from, to);
        TextRange {
            start: from,
            end: to,
        }
    }

    /// The range from some point over some length
    pub fn from_len(from: TextUnit, len: TextUnit) -> TextRange {
        TextRange::from_to(from, from + len)
    }

    /// The starting position of this range
    pub fn start(&self) -> TextUnit {
        self.start
    }

    /// The end position of this range
    pub fn end(&self) -> TextUnit {
        self.end
    }

    /// The length of this range
    pub fn len(&self) -> TextUnit {
        self.end - self.start
    }

    /// Is this range empty of any content?
    pub fn is_empty(&self) -> bool {
        self.start() == self.end()
    }
}

impl ops::Index<TextRange> for str {
    type Output = str;

    fn index(&self, index: TextRange) -> &str {
        &self[index.start().0 as usize..index.end().0 as usize]
    }
}

impl Serialize for TextUnit {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for TextUnit {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Deserialize::deserialize(deserializer)?;
        Ok(TextUnit(value))
    }
}

impl Serialize for TextRange {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        (self.start, self.end).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for TextRange {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let (start, end) = Deserialize::deserialize(deserializer)?;
        Ok(TextRange { start, end })
    }
}
