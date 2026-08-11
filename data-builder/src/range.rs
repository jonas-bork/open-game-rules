use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Range<T: PartialOrd> {
    from: T,
    to: T,
}

#[derive(thiserror::Error, Debug)]
#[error("Invalid range ({from:?} to {to:?}) because {from:?} is larger than {to:?}")]
pub struct InvalidRangeError<T> {
    pub from: T,
    pub to: T,
}

impl<T: PartialOrd> Range<T> {
    pub fn new(from: T, to: T) -> Result<Self, InvalidRangeError<T>> {
        if from < to {
            Ok(Self { from, to })
        } else {
            Err(InvalidRangeError { from, to })
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
