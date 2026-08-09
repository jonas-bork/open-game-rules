use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Range<T: PartialOrd> {
    from: T,
    to: T,
}

pub struct InvalidRangeError;
type Result<T> = std::result::Result<T, InvalidRangeError>;

impl<T: PartialOrd> Range<T> {
    pub fn new(from: T, to: T) -> Result<Self> {
        if from < to {
            Ok(Self { from, to })
        } else {
            Err(InvalidRangeError)
        }
    }

    pub fn from(&self) -> &T {
        &self.from
    }

    pub fn to(&self) -> &T {
        &self.to
    }

    pub fn into_inner(self) -> (T, T) {
        (self.from, self.to)
    }
}
