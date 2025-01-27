extern crate clap;

use std::io;

use clap::{Arg, Command};

use crossterm::event::{
    self, Event, KeyCode, KeyEvent, KeyEventKind
};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        Block,
        Paragraph,
        Widget
    },
    DefaultTerminal,
    Frame,
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
        Some(()) => {

            let mut terminal  = ratatui::init();
            let app_result = App::default().run(&mut terminal);

            // https://ratatui.rs/tutorials/counter-app/error-handling/

            if let Err(err) = ratatui::restore() {
                eprintln!(
                    "failed to restore terminal. Run `reset` or restart your terminal to recover: {}",
                    err
                );
            }

            app_result.expect("something wrong")
        }
    }

}



#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}


impl App {

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {

        match event::read()? {

            Event::Key(key_event)
                if key_event.kind == KeyEventKind:: Press =>
                {
                    self.handle_key_events(key_event)
                }

            _ => {}
        }

        Ok(())
    }

    fn handle_key_events(&mut self, key_event: KeyEvent) {

        match key_event.code {
            KeyCode::Char('q')  => self.exit(),
            KeyCode::Left       => self.decrement_counter(),
            KeyCode::Right      => self.increment_counter(),
            _                   => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }

}



impl Widget for &App {

    fn render(self, area: Rect, buf: &mut Buffer) {

        let title = Line::from("Counter app Tutorial".bold());

        let instructions = Line::from(vec![
            "Decrement".into(),
            "<Left>".blue().bold(),
            "Increment".into(),
            "<Right>".blue().bold(),
            "Quit".into(),
            "<Q>".blue().bold(),
        ]);

    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::THICK);

    let counter_text = Text::from(vec![Line::from(vec![
        "Value: ".into(),
        self.counter.to_string().yellow(),
    ])]);

    Paragraph::new(counter_text)
        .centered()
        .block(block)
        .render(area, buf);

    }

}


// https://ratatui.rs/tutorials/counter-app/basic-app/
