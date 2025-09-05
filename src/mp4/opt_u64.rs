use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Deref, DerefMut},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct OptU64(pub Option<u64>);

impl Deref for OptU64 {
    type Target = Option<u64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for OptU64 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Add for OptU64 {
    type Output = OptU64;

    fn add(self, rhs: Self) -> Self::Output {
        if let Some(a) = *self
            && let Some(b) = *rhs
        {
            Self(Some(a + b))
        } else {
            Self(None)
        }
    }
}

impl AddAssign for OptU64 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl PartialOrd for OptU64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OptU64 {
    fn cmp(&self, other: &Self) -> Ordering {
        match (**self, **other) {
            (None, None) => Ordering::Equal,
            (None, _) => Ordering::Greater,
            (_, None) => Ordering::Less,
            (Some(a), Some(b)) => a.cmp(&b),
        }
    }
}

impl PartialEq<u64> for OptU64 {
    fn eq(&self, other: &u64) -> bool {
        **self == Some(*other)
    }
}

impl PartialOrd<u64> for OptU64 {
    fn partial_cmp(&self, other: &u64) -> Option<Ordering> {
        Some(self.cmp(&OptU64(Some(*other))))
    }
}
