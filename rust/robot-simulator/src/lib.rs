// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

use Direction::*;

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, direction: d }
    }

    #[must_use]
    pub fn turn_right(mut self) -> Self {
        match self.direction {
            North => self.direction = East,
            East => self.direction = South,
            South => self.direction = West,
            West => self.direction = North,
        }
        self
    }

    #[must_use]
    pub fn turn_left(mut self) -> Self {
        match self.direction {
            North => self.direction = West,
            East => self.direction = North,
            South => self.direction = East,
            West => self.direction = South,
        }
        self
    }

    #[must_use]
    pub fn advance(mut self) -> Self {
        match self.direction {
            North => self.y += 1,
            East => self.x += 1,
            South => self.y -= 1,
            West => self.x -= 1,
        }
        self
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, char| match char {
            'R' => robot.turn_right(),
            'A' => robot.advance(),
            'L' => robot.turn_left(),
            _ => robot,
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
