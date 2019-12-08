use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn add(&mut self, position: &Position) {
        self.x += position.x;
        self.y += position.y;
    }

    pub fn clone(&self) -> Position {
        return Position {
            x: self.x,
            y: self.y,
        };
    }

    pub fn distance_from_origo(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}
impl Eq for Position {}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Position) -> Ordering {
        self.distance_from_origo().cmp(&other.distance_from_origo())
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
