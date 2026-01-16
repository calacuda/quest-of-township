use std::ops::{Add, Sub};

use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;

pub type CoordNum = i32;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Component)]
pub struct PlayerLoc(pub GridCoords);
// pub struct PlayerLoc(pub usize, pub usize);

impl Sub<(CoordNum, CoordNum)> for PlayerLoc {
    type Output = PlayerLoc;

    fn sub(self, rhs: (CoordNum, CoordNum)) -> Self::Output {
        // Self(self.0 - rhs.0, self.1 - rhs.1)
        Self(GridCoords::new(self.0.x - rhs.0, self.0.y - rhs.1))
    }
}

impl Add<(CoordNum, CoordNum)> for PlayerLoc {
    type Output = PlayerLoc;

    fn add(self, rhs: (CoordNum, CoordNum)) -> Self::Output {
        // Self(self.0 + rhs.0, self.1 + rhs.1)
        Self(GridCoords::new(self.0.x + rhs.0, self.0.y + rhs.1))
    }
}

impl From<PlayerLoc> for Vec2 {
    fn from(value: PlayerLoc) -> Self {
        Vec2::from((value.0.x as f32, value.0.y as f32))
    }
}
