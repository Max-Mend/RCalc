use std::{io, time::{Duration, Instant}};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use ratatui::layout::{Layout, Constraint, Direction};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, Paragraph, List, ListItem};
use ratatui::text::{Line, Span};
use crate::func;

fn hex_to_rgb(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    Color::Rgb(r, g, b)
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut input = String::new();
    let mut history: Vec<String> = Vec::new();
    let mut result_message = String::from("Enter expression or command (help for info)");
    let mut temp_message: Option<(String, Instant)> = None;

    loop {
        if let Some((msg, start_time)) = &temp_message {
            if start_time.elapsed() >= Duration::from_millis(500) {
                temp_message = None;
            }
        }

        let display_message = if let Some((msg, _)) = &temp_message {
            msg.clone()
        } else {
            result_message.clone()
        };

        terminal.draw(|frame| {
            let area = frame.area();

            let main_block = Block::bordered()
                .title("RCalc")
                .border_style(Style::default().fg(Color::Red))
                .title_style(Style::default().fg(Color::Red).bold());
            frame.render_widget(main_block.clone(), area);
            let inner = main_block.inner(area);

            let vertical = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(1),
                    Constraint::Length(3),
                    Constraint::Length(1)
                ])
                .split(inner);

            let columns = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(70),
                    Constraint::Percentage(30),
                ])
                .split(vertical[0]);

            let calc_text = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled("Result: ", Style::default().fg(Color::Yellow)),
                    Span::raw(&display_message),
                ]),
            ];
            let calc_block = Block::bordered()
                .title("Calc")
                .style(Style::default()
                    .bg(hex_to_rgb("#1E1E1E"))
                    .fg(Color::Red));
            let calc_para = Paragraph::new(calc_text).block(calc_block);
            frame.render_widget(calc_para, columns[0]);

            let history_items: Vec<ListItem> = history
                .iter()
                .rev()
                .take(20)
                .map(|h| ListItem::new(h.clone()))
                .collect();

            let history_list = List::new(history_items)
                .block(Block::bordered()
                    .title("History")
                    .style(Style::default()
                        .bg(hex_to_rgb("#1E1E1E"))
                        .fg(Color::Red)));
            frame.render_widget(history_list, columns[1]);

            let input_block = Block::bordered()
                .title("Input")
                .style(Style::default().fg(Color::Green));
            let input_para = Paragraph::new(input.as_str())
                .block(input_block);
            frame.render_widget(input_para, vertical[1]);

            let footer = Paragraph::new("Q - quit | Enter - calculate | Backspace - delete | C - clear")
                .dim()
                .style(Style::default().fg(Color::Red));
            frame.render_widget(footer, vertical[2]);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('c') => {
                        history.clear();
                        temp_message = Some(("History cleared".to_string(), Instant::now()));
                    }
                    KeyCode::Char(ch) => {
                        input.push(ch);
                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Enter => {
                        if !input.is_empty() {
                            let input_trimmed = input.trim();

                            match input_trimmed {
                                "help" => {
                                    result_message = "Commands: +, -, *, /, ** (power), // (sqrt), info, exit, c (clear)".to_string();
                                    history.push(format!("> help"));
                                }
                                "info" => {
                                    result_message = "RCalc v1.1 | Rust Calculator | Author: Max-Mend".to_string();
                                    history.push(format!("> info"));
                                }
                                "exit" => break,
                                _ => {
                                    match func::parse_and_calculate(input_trimmed) {
                                        Ok(res) => {
                                            result_message = format!("{}", res);
                                            history.push(format!("{} = {}", input_trimmed, res));
                                        }
                                        Err(err) => {
                                            result_message = err;
                                        }
                                    }
                                }
                            }

                            input.clear();
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}