use std::fmt::Debug;
use std::ops::Range;

#[derive(Copy, Clone, PartialEq)]
pub struct Span {
    start: usize,
    end: usize
}

impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}..{}]", self.start, self.end)
    }
}

impl From<Range<usize>> for Span {
    fn from(value: Range<usize>) -> Self {
        Self {
            start: value.start,
            end: value.end
        }
    }
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end
        }
    }
    pub fn extract_from_str<'a>(&self, string: &'a str) -> &'a str {
        &string[self.start..self.end]
    }
}