use std::process::Command;
use std::thread;
use std::time::Duration;
use crate::univers::{Universe};
use crate::univers::State::{Alive, Dead};

mod univers;
mod tests;

fn main() {
    // our univers only square 5x5,4x4,3x3...
    clear_terminal();

    let mut universe = Universe::generate(10, 30);

    loop {
        display_universe(&universe);
        universe = Universe::calculate_future(&universe);
        thread::sleep(Duration::from_millis(100));
    }

}

fn clear_terminal() ->() {
    Command::new("clear").status().expect("failed to clear terminal");
}

fn display_universe(universe: &Universe) -> (){

    let mut display = String::new();

    for line in universe.universe.iter() {

        let mut display_line = String::new();

        for cell in line.iter() {
            display_line.push_str(match cell.state {
                Alive => " o ",
                Dead => "   "
            });
        }
        display.push_str(&display_line);
        display.push('\n');

    }

    clear_terminal();
    println!("{}", display);

}