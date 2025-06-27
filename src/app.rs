use color_eyre::eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout},
    style::{Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, Paragraph},
    DefaultTerminal, Frame,
};

pub struct App {
    messages: Vec<String>,
    input: String,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            input: String::from("None"),
            messages: Vec::new(),
            exit: false,
        }
    }

    fn check_input(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => self.exit = true,
                KeyCode::Esc => self.exit = true,
                _ => {}
            }
        }
        Ok(())
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.check_input()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let vertical = Layout::vertical([
            Constraint::Min(1),
            Constraint::Length(3),
            Constraint::Length(1),
        ]);
        let [messages_area, input_area, help_area] = vertical.areas(frame.area());

        let msg = vec!["Exit ".into(), "<Esc/q>".blue().bold()];

        let style = Style::default().add_modifier(Modifier::RAPID_BLINK);

        let text = Text::from(Line::from(msg)).patch_style(style);
        let help_message = Paragraph::new(text).centered();

        frame.render_widget(help_message, help_area);

        let input = Paragraph::new(self.input.as_str())
            .style(Style::default())
            .block(Block::bordered().title("Input"));

        frame.render_widget(input, input_area);

        let messages: Vec<ListItem> = self
            .messages
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let content = Line::from(Span::raw(format!("{i}: {m}")));
                ListItem::new(content)
            })
            .collect();
        let messages = List::new(messages).block(Block::bordered().title("Messages"));
        frame.render_widget(messages, messages_area);
    }
}
