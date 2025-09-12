use bevy::prelude::Component;

#[derive(Component)]
pub struct Column {
    /// The column screen this refers to.
    /// Represents the unique identifier for the column.
    num: usize,

    /// The vertical position of the column on the screen.
    ///
    /// The screen consists of two buffers:
    /// - The `end screen` buffer, which spans from 0 (inclusive) to `position`,
    ///   representing the destination screen.
    /// - The `start screen` buffer, which spans from `position` (inclusive) to 200,
    ///   representing the starting screen.
    ///
    /// The `position` value then determines the boundary between these two buffers.
    position: usize,

    /// The time to wait before the column starts moving.
    /// A countdown timer used to delay the column's movement.
    wait: u8,
}

impl Column {
    pub fn new(num: usize, wait: u8) -> Self {
        Self {
            num,
            position: 0,
            wait,
        }
    }

    pub fn num(&self) -> usize {
        self.num
    }

    pub fn position(&self) -> usize {
        self.position
    }

    /// Updates the column's position based on the current state.
    ///
    /// The column will move if the wait time has elapsed. If the column is ready to move,
    /// it will increase its position based on the current position and some predefined
    /// rules (e.g., based on the column's current position).
    ///
    /// # Returns
    /// `true` if the column has moved, `false` if it was not ready to move
    pub fn update_position(&mut self) -> bool {
        if self.wait > 0 {
            // Not ready to move.
            self.wait -= 1;
            return false;
        }
        let pos = self.position;
        let mut dy = if pos < 16 { pos + 1 } else { 8 };
        if pos + dy >= 200 {
            dy = 200 - pos;
        }
        self.position += dy;
        true
    }
}
