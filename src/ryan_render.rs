use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use std::io::{Stdout, Write};
use crate::ryan_frame::Frame;

// TODO: Define the 'render' function that takes a mutable reference to Stdout, 
//       a reference to the last frame, a reference to the current frame, and a boolean 'force'
pub fn render(stdout: &mut Stdout, prev_frame: &Frame, curr_frame: &Frame, force: bool){
    // TODO: If 'force' is true, clear the terminal and set the background and foreground colors

    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
        stdout.queue(SetForegroundColor(Color::White)).unwrap();
    }

    // TODO: Iterate over the current frame with indices
    for (x, col) in curr_frame.iter().enumerate(){
        for (y, character) in col.iter().enumerate(){
            // TODO: For each element in the current frame, check if it differs from the corresponding element 
            //       in the last frame or if 'force' is true 
            
            // TODO: If the element has changed or 'force' is true, move the cursor to the appropriate position 
            //       and print the element
            
            if *character != prev_frame[x][y] || force{
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *character);
            }

        }
    }

    stdout.flush().unwrap();

}





// TODO: Flush the stdout to apply all queued commands