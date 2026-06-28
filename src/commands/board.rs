use std::io;
use std::time::Duration;

use crate::error::Result;
use crate::storage::{TaskStore, resolve_storage};
use crate::task::TaskStatus;
use crossterm::ExecutableCommand;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
pub fn run(global: bool) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let ctx = resolve_storage(global, &cwd)?;
    let mut store = TaskStore::open(&ctx)?;

    let mut stdout = io::stdout();
    enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;

    let result = run_loop(&mut store);

    disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;

    result
}

fn run_loop(store: &mut TaskStore) -> Result<()> {
    let mut terminal = ratatui::init();
    let mut selected_open = 0usize;
    let mut selected_done = 0usize;
    let mut focus_open = true;

    loop {
        let open_tasks: Vec<_> = store
            .tasks()
            .iter()
            .filter(|t| t.status == TaskStatus::Open)
            .collect();
        let done_tasks: Vec<_> = store
            .tasks()
            .iter()
            .filter(|t| t.status == TaskStatus::Done)
            .collect();

        if selected_open >= open_tasks.len().max(1) {
            selected_open = open_tasks.len().saturating_sub(1);
        }
        if selected_done >= done_tasks.len().max(1) {
            selected_done = done_tasks.len().saturating_sub(1);
        }

        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(3), Constraint::Length(3)])
                .split(frame.area());

            let cols = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(chunks[0]);

            let open_items: Vec<ListItem> = open_tasks
                .iter()
                .map(|t| {
                    let id = &t.id[..8.min(t.id.len())];
                    ListItem::new(Line::from(vec![
                        Span::styled(format!("{id} "), Style::default().fg(Color::Cyan)),
                        Span::raw(t.title.as_str()),
                    ]))
                })
                .collect();

            let done_items: Vec<ListItem> = done_tasks
                .iter()
                .map(|t| {
                    let id = &t.id[..8.min(t.id.len())];
                    ListItem::new(Line::from(vec![
                        Span::styled(format!("{id} "), Style::default().fg(Color::Green)),
                        Span::raw(t.title.as_str()),
                    ]))
                })
                .collect();

            let open_border = if focus_open {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            let done_border = if !focus_open {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };

            let open_list = List::new(open_items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(open_border)
                        .title(format!(" Open ({}) ", open_tasks.len())),
                )
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol("▸ ");

            let done_list = List::new(done_items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(done_border)
                        .title(format!(" Done ({}) ", done_tasks.len())),
                )
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol("▸ ");

            let mut open_state =
                ratatui::widgets::ListState::default().with_selected(Some(selected_open));
            let mut done_state =
                ratatui::widgets::ListState::default().with_selected(Some(selected_done));

            frame.render_stateful_widget(open_list, cols[0], &mut open_state);
            frame.render_stateful_widget(done_list, cols[1], &mut done_state);

            let help = Paragraph::new(Line::from(
                "j/k:nav  Tab:column  d:done  q:quit  |  add: shipflow add \"task\"",
            ))
            .block(Block::default().borders(Borders::ALL).title(" Keys "));
            frame.render_widget(help, chunks[1]);
        })?;

        if event::poll(Duration::from_millis(200))?
            && let Event::Key(key) = event::read()?
        {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Tab => focus_open = !focus_open,
                KeyCode::Char('j') | KeyCode::Down => {
                    if focus_open {
                        selected_open = (selected_open + 1).min(open_tasks.len().saturating_sub(1));
                    } else {
                        selected_done = (selected_done + 1).min(done_tasks.len().saturating_sub(1));
                    }
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    if focus_open {
                        selected_open = selected_open.saturating_sub(1);
                    } else {
                        selected_done = selected_done.saturating_sub(1);
                    }
                }
                KeyCode::Char('d') if focus_open && !open_tasks.is_empty() => {
                    let id = open_tasks[selected_open].id.clone();
                    let _ = store.mark_done(&id, None);
                }
                KeyCode::Char('a') => {
                    // Inline add deferred to v0.2; hint shown in footer.
                }
                _ => {}
            }
        }
    }

    ratatui::restore();
    Ok(())
}
