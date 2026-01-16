use std::ops::{Add, Sub};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, Debug, Component, Deserialize, Serialize,
)]
pub struct GridLoc {
    pub x: i32,
    pub y: i32,
}

impl Sub<(i32, i32)> for GridLoc {
    type Output = Self;

    fn sub(self, rhs: (i32, i32)) -> Self::Output {
        Self {
            x: self.x - rhs.0,
            y: self.y - rhs.1,
        }
    }
}

impl Add<(i32, i32)> for GridLoc {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        // Self(self.0 + rhs.0, self.1 + rhs.1)
        Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl From<GridLoc> for Vec2 {
    fn from(value: GridLoc) -> Self {
        Vec2::from((value.x as f32, value.y as f32))
    }
}
