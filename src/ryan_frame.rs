use crate::{NUM_COLS, NUM_ROWS};

// TODO: Define a type alias 'Frame' as a 2D array of characters with dimensions NUM_ROWS x NUM_COLS
pub type Frame = [[char; NUM_ROWS]; NUM_COLS];  

// TODO: Implement a function 'new_frame' that returns a new Frame initialized with spaces (' ')
pub fn new_frame() -> Frame{
    [[' '; NUM_ROWS]; NUM_COLS]
}

// TODO: Define a trait 'Drawable' with a method 'draw' that takes a mutable reference to a Frame

pub trait Drawable{
    fn draw (&self, frame: &mut Frame);
}