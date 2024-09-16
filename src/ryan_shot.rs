// TODO: Import the necessary items from the ryan_frame module (Drawable, Frame)
// TODO: Import the Duration struct from the std::time module
// TODO: Import the Timer struct from the rusty_time crate
use crate::ryan_frame::{Drawable, Frame};
use rusty_time::Timer;
use std::time::Duration;

// TODO: Define the Shot struct with the following fields:
//       - x: the x-coordinate of the shot (usize)
//       - y: the y-coordinate of the shot (usize)
//       - exploding: a boolean indicating if the shot is exploding
//       - timer: a Timer object to manage the shot's timing

pub struct Shot{
    pub x: usize,
    pub y: usize, 
    pub exploding: bool,
    timer: Timer,
}

// TODO: Implement the Shot struct with the following methods:
//       - new: a constructor that initializes a new Shot instance with given x and y coordinates, 
//              sets exploding to false, and initializes the timer with a duration of 50 milliseconds
//       - update: a method that updates the shot's position and state based on the given delta duration
//                 - Tick the timer with the given delta duration
//                 - If the timer is finished and the shot is not exploding, decrement the y-coordinate if it's greater than 0
//                 - Reset the timer
//       - explode: a method that sets the shot to exploding state
//                 - Set the exploding field to true
//                 - Reset the timer with a new duration of 250 milliseconds
//       - dead: a method that checks if the shot is dead
//                 - Return true if the shot is exploding and the timer is finished, or if the y-coordinate is 0

impl Shot{
    pub fn new(x: usize,y: usize) -> Self {
        Self{
            x,
            y, 
            exploding: false, 
            timer: Timer::new(Duration::from_millis(50)), 
        }
    }

    pub fn update(&mut self, delta: Duration){
        self.timer.tick(delta);
        if self.timer.finished() && !self.exploding{
            if self.y > 0{
                self.y -= 1;
            }
            self.timer.reset();
        }
    }

    pub fn explode(&mut self){
        self.exploding = true;
    }

    pub fn dead(&self) -> bool{
        if (self.exploding && self.timer.finished()) || self.y == 0{
            return true
        } 
        
        return false
    }
}


// TODO: Implement the Drawable trait for the Shot struct with the following method:
//       - draw: a method that draws the shot onto a Frame
//                 - If the shot is exploding, draw '*' at the shot's coordinates
//                 - Otherwise, draw '|' at the shot's coordinates

impl Drawable for Shot{
    fn draw (&self, frame: &mut Frame){
        if self.exploding{
            frame[self.x][self.y] = '*';
        }else{
            frame[self.x][self.y] = '|';
        }
    }
}