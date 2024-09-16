// TODO: Import the necessary items from the ryan_frame module (Drawable, Frame)
// TODO: Import the Timer struct from the rusty_time crate
// TODO: Import the Duration struct from the std::time module

use crate::ryan_frame::{Drawable, Frame};
use crate::{NUM_COLS, NUM_ROWS};
use rusty_time::Timer;
use std::{cmp::max, time::Duration};


// TODO: Define the Invader struct with the following fields:
//       - x: the x-coordinate of the invader (usize)
//       - y: the y-coordinate of the invader (usize)
//       - points: a u16 representing the points awarded for killing the invader

pub struct Invader{
    x: usize, 
    y: usize, 
    points: u16, 
}

// TODO: Define the Invaders struct with the following fields:
//       - army: a vector of Invader instances
//       - total_count: the total number of invaders (usize)
//       - move_timer: a Timer object to manage the movement timing
//       - direction: an i32 indicating the current movement direction (1 for right, -1 for left)

pub struct Invaders{
    army: Vec<Invader>,
    total_count: usize, 
    move_timer: Timer,
    direction: i32,
}


// TODO: Implement the Invaders struct with the following methods:
//       - new: a constructor that initializes a new Invaders instance with a grid of invaders
//       - update: a method that updates the invaders' positions based on the given delta duration
//                 - Tick the move_timer with the given delta duration
//                 - If the move_timer is finished, reset it and determine the movement direction
//                 - Move the invaders downwards if necessary and adjust the move_timer duration
//                 - Move the invaders left or right based on the current direction
//                 - Return true if the invaders moved, otherwise return false
//       - all_killed: a method that returns true if all invaders are killed (army is empty)
//       - reached_bottom: a method that returns true if any invader has reached the bottom of the screen
//       - kill_invader_at: a method that removes an invader at the given x and y coordinates and returns the points awarded

impl Invaders{
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 0 .. NUM_COLS{
            for y in 0 .. NUM_ROWS{
                if (x > 1)
                && (x < NUM_COLS - 2)
                && (y > 0)
                && (y < 9)
                && (x % 2 == 0)
                && (y % 2 == 0)
                {
                    army.push(Invader{x, y, points: 1});
                }
            } 
        }
        let total_count: usize = army.len();
        Self{
            army, 
            total_count,
            move_timer: Timer::new(Duration::from_millis(2000)), 
            direction: 1,
        }
    }

    pub fn update(&mut self, delta: Duration) -> bool {
       self.move_timer.tick(delta);
       if self.move_timer.finished(){
          self.move_timer.reset();
          let mut downwards = false;
            if self.direction == -1 {
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                if min_x == 0 {
                    self.direction = 1;
                    downwards = true;
                }
            } else {
                let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
                if max_x == NUM_COLS - 1 {
                    self.direction = -1;
                    downwards = true;
                }
            }
            if downwards {
                let new_duration = max(self.move_timer.duration().as_millis() - 250, 250);
                self.move_timer.set_duration(Duration::from_millis(new_duration as u64));
                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            } else {
                for invader in self.army.iter_mut() {
                    invader.x = ((invader.x as i32) + self.direction) as usize;
                }
            }
            return true;
       }
       return false 
    }

    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }

    pub fn reached_bottom(&self) -> bool {
        self.army.iter().any(|invader| invader.y >= NUM_ROWS - 1)
    }

    pub fn kill_invader_at(&mut self, x: usize, y: usize) -> u16 {
        if let Some(idx) = self
            .army
            .iter()
            .position(|invader| (invader.x == x) && (invader.y == y))
        {
            let points = self.army[idx].points;
            self.army.remove(idx);
            points
        } else {
            0
        }
    }
}

// TODO: Implement the Default trait for the Invaders struct, delegating to the new method

impl Default for Invaders{
    fn default() -> Self{
        Invaders::new() 
    }
}

// TODO: Implement the Drawable trait for the Invaders struct with the following method:
//       - draw: a method that draws the invaders onto a Frame
//                 - Draw 'x' or '+' based on the remaining time in the move_timer

impl Drawable for Invaders{
    fn draw(&self, frame: &mut Frame){
        for invader in self.army.iter(){
            frame[invader.x][invader.y] = if (self.move_timer.remaining().as_secs_f32()
            / self.move_timer.duration().as_secs_f32())
            > 0.5
        {
            'x'
        } else {
            '+'
        }
        }
    } 
}