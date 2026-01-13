use std::ops::{Add, Sub};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, Debug, Component, Deserialize, Serialize,
)]
pub struct PlayerLoc(pub usize, pub usize);

impl Sub<(usize, usize)> for PlayerLoc {
    type Output = PlayerLoc;

    fn sub(self, rhs: (usize, usize)) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Add<(usize, usize)> for PlayerLoc {
    type Output = PlayerLoc;

    fn add(self, rhs: (usize, usize)) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl From<PlayerLoc> for Vec2 {
    fn from(value: PlayerLoc) -> Self {
        Vec2::from((value.0 as f32, value.1 as f32))
    }
}
