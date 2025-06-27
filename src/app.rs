use color_eyre::eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    layout::{Constraint, Layout},
    style::{Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, Paragraph},
    DefaultTerminal, Frame,
};

const TOTAL_MSG: usize = 13;

pub struct App {
    messages: Vec<String>,
    offset: usize,
    edit_mode: bool,
    input: String,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            offset: 0,
            edit_mode: false,
            messages: Vec::new(),
            exit: false,
        }
    }

    fn add_offset(&mut self) {
        let surpass_max_msg = self.messages.len() > TOTAL_MSG;
        let not_first_msg = self.offset > 0;
        if surpass_max_msg && not_first_msg {
            self.offset -= 1;
        }
    }

    fn sub_offset(&mut self) {
        let surpass_max_msg = self.messages.len() > TOTAL_MSG;
        let not_last_msg = ((self.offset + 1) + TOTAL_MSG) < self.messages.len();
        if surpass_max_msg && not_last_msg {
            self.offset += 1;
        }
    }

    fn default_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('e') => self.edit_mode = true,
            KeyCode::Char('E') => self.edit_mode = true,
            KeyCode::Up => self.add_offset(),
            KeyCode::Down => self.sub_offset(),
            KeyCode::Esc => self.exit = true,
            _ => {}
        }
    }

    fn edit_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                self.input.clear();
                self.edit_mode = false;
            }
            KeyCode::Esc => {
                self.input.clear();
                self.edit_mode = false;
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Enter => {
                let new_ingress = self.input.clone();
                self.messages.push(new_ingress);
                self.input.clear();
                self.edit_mode = false;
            }
            event::KeyCode::Char(char) => self.input.push(char),
            _ => {}
        }
    }

    fn check_input(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if self.edit_mode {
                self.edit_input(key);
            } else {
                self.default_input(key);
            }
        }
        Ok(())
    }

    fn get_help_message(&mut self) -> Vec<Span<'_>> {
        if self.edit_mode {
            vec!["Go back ".into(), "<Esc/q>".blue().bold()]
        } else {
            vec![
                "Exit ".into(),
                "<Esc/q>".blue().bold(),
                " Edit mode ".into(),
                "<e/E>".blue().bold(),
            ]
        }
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

        // Rendering MESSAGES
        let messages: Vec<ListItem> = self
            .messages
            .iter()
            .enumerate()
            .filter(|(i, _)| *i >= self.offset)
            .map(|(i, m)| {
                let content = Line::from(Span::raw(format!("{i}: {m}")));
                ListItem::new(content)
            })
            .collect();
        let messages = List::new(messages).block(Block::bordered().title("Messages"));
        frame.render_widget(messages, messages_area);

        // Render INPUT
        let style_input = if self.edit_mode {
            Style::default().fg(ratatui::style::Color::Red)
        } else {
            Style::default()
        };

        let input = Paragraph::new(self.input.as_str())
            .style(Style::default().fg(ratatui::style::Color::White))
            .block(Block::bordered().title("Input").border_style(style_input));

        frame.render_widget(input, input_area);

        // Render HELP MESSAGE
        let msg = self.get_help_message();

        let style = Style::default().add_modifier(Modifier::RAPID_BLINK);

        let text = Text::from(Line::from(msg)).patch_style(style);
        let help_message = Paragraph::new(text).centered();
        frame.render_widget(help_message, help_area);
    }
}
