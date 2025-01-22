extern crate clap;

use std::io;

use clap::{Arg, Command};

use crossterm::event::{
    self, Event, KeyCode, KeyEvent, KeyEventKind
};

use ratatui::{

};

fn main() {
    let matches =
        Command::new("First test program")
            .version("0.0.1")
            .about("first terminal app")
            .arg(
                Arg::new("tui")
                    .short('t')
                    .long("terminal_ui")
                    .help("open terminal user interface")
                    .action(clap::ArgAction::SetTrue)
            )
            .get_matches();

    let tui_flag: Option<()> = matches
        .get_one::<bool>("tui")
        .copied()
        .filter(|&called| called)
        .map(|_| ());


    match tui_flag {
        None => println!("No value given"),
        Some(()) => println!("flag called"),
    }
}
