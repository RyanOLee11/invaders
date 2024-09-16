// TODO: Import the necessary items from the ryan_frame module (Drawable, Frame)

use crate::ryan_frame::{Drawable, Frame};

// TODO: Define the Menu struct with the following fields:
//       - options: a vector of strings representing menu options
//       - selection: an index representing the currently selected option

pub struct Menu{
    pub options: Vec<String>,
    pub selection: usize,
}

// TODO: Implement the Menu struct with the following methods:
//       - new: a constructor that initializes the Menu with default options and selection
//       - change_option: a method that changes the selected option based on a boolean input

impl Menu {
    pub fn new() -> Self{
        Self{
            options: vec![String::from("New Game"), String::from("Exit")], 
            selection: 0,
        }
    }

    pub fn change_option(&mut self, up: bool){
        if up && self.selection > 0 {
            self.selection -= 1;
        }else if !up && self.selection < self.options.len() - 1{
            self.selection +=1;
        }
    }
}

// TODO: Implement the Default trait for the Menu struct, delegating to the new method

impl Default for Menu{
    fn default() -> Self{
        Self::new()
    }
}

// TODO: Implement the Drawable trait for the Menu struct with the following method:
//       - draw: a method that draws the menu options onto a Frame, indicating the selected option

impl Drawable for Menu{
    fn draw (&self, frame: &mut Frame){
        frame[0][self.selection] = '>';
        for (i, option) in self.options.iter().enumerate(){
            for x in 0..option.len(){
                frame[x+1][i] = self.options[i].chars().nth(x).unwrap();   
            }
        }
    }
}