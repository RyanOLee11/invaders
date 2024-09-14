use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rusty_audio::Audio;
use std::{
    error::Error,
    sync::mpsc::{self, Receiver},
    time::{Duration, Instant},
    {io, thread},
};
/*
use invaders::{
    frame::{self, new_frame, Drawable, Frame},
    invaders::Invaders,
    level::Level,
    menu::Menu,
    player::Player,
    render,
    score::Score,
};
*/
fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "sounds/explode.wav");
    audio.add("lose", "sounds/lose.wav");
    audio.add("move", "sounds/move.wav");
    audio.add("pew", "sounds/pew.wav");
    audio.add("startup", "sounds/startup.wav");
    audio.add("win", "sounds/win.wav");
    audio.play("startup");

    //terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen);
    stdout.execute(Hide)?;

    //Game loop
    'gameloop: loop

    // Cleanup code
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}