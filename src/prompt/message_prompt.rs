use crate::Config;
use crate::TermBuffer;
use crossterm::{self as ct, InputEvent, KeyEvent};

#[derive(Debug)]
pub struct MessagePrompt<'a> {
    config: &'a mut Config,
    input: Vec<String>,
    cursor: (u16, u16),
}

pub enum MessagePromptResult {
    Message(String),
    Escape,
    Terminate,
}

impl<'a> MessagePrompt<'a> {
    pub fn new(config: &'a mut Config) -> Self {
        MessagePrompt {
            config,
            input: vec![String::new()],
            cursor: (0, 0),
        }
    }

    pub fn run(mut self) -> MessagePromptResult {
        let mut buffer = TermBuffer::new();

        let input = crossterm::input();
        let mut sync_stdin = input.read_sync();

        let mut first_iteration = true;

        loop {
            let event = if first_iteration {
                first_iteration = false;
                None
            } else {
                sync_stdin.next()
            };

            match event {
                Some(InputEvent::Keyboard(KeyEvent::Ctrl('c'))) => {
                    return MessagePromptResult::Terminate;
                }
                Some(InputEvent::Keyboard(KeyEvent::Ctrl('a'))) => {
                    self.cursor.0 = 0;
                }
                Some(InputEvent::Keyboard(KeyEvent::Ctrl('e'))) => {
                    let (_, y) = self.cursor;
                    let line = self
                        .input
                        .get(y as usize)
                        .expect("ctrl-e unable to find current line");
                    self.cursor.0 = line.len() as u16;
                }
                Some(InputEvent::Keyboard(KeyEvent::Alt('\n')))
                | Some(InputEvent::Keyboard(KeyEvent::Ctrl('\n'))) => {
                    self.input.push(String::new());
                    self.cursor.1 += 1;
                }
                Some(InputEvent::Keyboard(KeyEvent::Char('\n'))) => {
                    return MessagePromptResult::Message(self.input.join("\n"));
                }
                Some(InputEvent::Keyboard(KeyEvent::Char(c))) => {
                    let (x, y) = self.cursor;
                    let line = self.input.get_mut(y as usize).unwrap();
                    line.insert(x as usize, c);
                    self.cursor.0 += 1;
                }
                Some(InputEvent::Keyboard(KeyEvent::Left)) => {
                    self.cursor.0 = self.cursor.0.saturating_sub(1);
                }
                Some(InputEvent::Keyboard(KeyEvent::Right)) => {
                    let (x, y) = self.cursor;
                    let line = self.input.get_mut(y as usize).expect("KE::Right get_mut");
                    if line.len() < x as usize + 1 {
                        line.push(' ');
                    }
                    self.cursor.0 += 1;
                }
                Some(InputEvent::Keyboard(KeyEvent::Up)) => {
                    self.cursor.1 = self.cursor.1.saturating_sub(1);
                }
                Some(InputEvent::Keyboard(KeyEvent::Down)) => {
                    let (_, y) = self.cursor;
                    if (y as usize) + 1 >= self.input.len() {
                        self.input.push(String::new());
                    }
                    self.cursor = (0, y + 1);
                }
                Some(InputEvent::Keyboard(KeyEvent::Backspace)) => match self.cursor {
                    (0, 0) => {}
                    (0, y) => {
                        self.input.remove(y as usize);
                        self.cursor.1 -= 1;
                        self.cursor.0 = self.input[self.cursor.1 as usize].len() as u16;
                    }
                    (x, y) => {
                        let line = self
                            .input
                            .get_mut(y as usize)
                            .expect("Backspace (x, y) get line y");

                        if x as usize >= line.len() {
                            line.pop();
                        } else {
                            line.remove(x as usize - 1);
                        }
                        self.cursor.0 -= 1;
                    }
                },
                Some(InputEvent::Keyboard(KeyEvent::Esc)) => {
                    return MessagePromptResult::Escape;
                }
                _ => {}
            };

            let (x, y) = self.cursor;
            let instructions = "Commit message (arrow keys for multiple lines):";
            let divider = "-".repeat(instructions.len());
            buffer.push_line(instructions);
            buffer.push_line(divider);

            // The offset for where the editor begins, i.e. the number of push_line calls above.
            let editor_y = 2;

            for (i, line) in self.input.iter().enumerate() {
                if i == 0 && line.len() > 50 {
                    let (good, bad) = crate::string::split_at(&line, 50);
                    buffer.push_line(format!(
                        "{}{}{}",
                        good,
                        ct::style(bad).with(ct::Color::Red),
                        crate::color::reset_display(),
                    ));
                } else {
                    buffer.push_line(line.to_string());
                }
            }

            buffer.set_next_cursor((x, y + editor_y));
            buffer.render_frame();
            buffer.flush();
        }
    }
}