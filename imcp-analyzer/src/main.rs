//! IMCP Protocol Analyzer
//!
//! This tool provides a TUI interface for analyzing IMCP protocol messages in real-time.
//! It captures messages from a WebSocket connection and displays them in an interactive interface.

use anyhow::{Context, Result};
use chrono::Local;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use imcp::protocol::{Message, MessageType};
use serde_json::to_string_pretty;
use std::{
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

mod network;

/// Represents a captured IMCP message with additional metadata
#[derive(Debug)]
struct MessageRecord {
    /// Timestamp when the message was received
    timestamp: chrono::DateTime<Local>,
    /// The actual IMCP message
    message: Message,
    /// Direction of the message (incoming or outgoing)
    direction: MessageDirection,
}

/// Indicates the direction of a message in the communication flow
#[derive(Debug)]
enum MessageDirection {
    /// Message received from a remote peer
    Incoming,
    /// Message sent to a remote peer
    Outgoing,
}

/// Main application state
struct App {
    /// List of captured messages
    messages: Vec<MessageRecord>,
    /// Index of the currently selected message in the list
    selected: Option<usize>,
    /// Flag to indicate if the application should quit
    should_quit: bool,
}

impl App {
    /// Creates a new instance of the application
    fn new() -> Self {
        Self {
            messages: Vec::new(),
            selected: None,
            should_quit: false,
        }
    }

    /// Adds a new message to the application state
    fn add_message(&mut self, message: Message, direction: MessageDirection) {
        self.messages.push(MessageRecord {
            timestamp: Local::now(),
            message,
            direction,
        });
    }

    /// Formats a message for display in the list view
    fn format_message(&self, record: &MessageRecord) -> String {
        let direction = match record.direction {
            MessageDirection::Incoming => "←",
            MessageDirection::Outgoing => "→",
        };
        let timestamp = record.timestamp.format("%H:%M:%S%.3f");
        let message_type = match record.message.header.message_type {
            MessageType::Handshake => "Handshake",
            MessageType::Data => "Data",
            MessageType::Control => "Control",
            MessageType::Heartbeat => "Heartbeat",
            MessageType::Error => "Error",
        };
        format!(
            "{} {} [{}] Session: {}",
            direction, timestamp, message_type, record.message.header.session_id
        )
    }

    /// Formats a message for display in the details view
    fn format_message_details(&self, record: &MessageRecord) -> String {
        let json = to_string_pretty(&record.message).unwrap_or_else(|_| "Invalid JSON".to_string());
        format!(
            "{}\n\nMessage Details:\n{}",
            self.format_message(record),
            json
        )
    }
}

/// Main entry point of the application
fn main() -> Result<()> {
    // Initialize terminal in raw mode and enter alternate screen
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create channels for inter-thread communication
    let (message_tx, message_rx) = mpsc::channel(); // For network messages
    let (input_tx, input_rx) = mpsc::channel(); // For user input

    // Start network listener in a separate thread
    let port = 8765; // Default IMCP port
    let network_tx = message_tx.clone();
    thread::spawn(move || {
        if let Err(e) = network::start_listener(port, network_tx) {
            eprintln!("Network listener error: {}", e);
        }
    });

    // Create app instance and run the main loop
    let app = App::new();
    let res = run_app(&mut terminal, app, message_rx, input_tx, input_rx);

    // Clean up terminal state
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

/// Main application loop
fn run_app<B: tui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    message_rx: mpsc::Receiver<(Message, bool)>,
    input_tx: mpsc::Sender<event::KeyCode>,
    input_rx: mpsc::Receiver<event::KeyCode>,
) -> Result<()> {
    let tick_rate = Duration::from_millis(200);

    // Spawn a thread to handle user input
    let input_tx_clone = input_tx.clone();
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            // Poll for user input events
            if event::poll(Duration::from_millis(100)).unwrap() {
                if let Event::Key(key) = event::read().unwrap() {
                    input_tx_clone.send(key.code).unwrap();
                }
            }
            // Send periodic tick events
            if last_tick.elapsed() >= tick_rate {
                input_tx_clone.send(event::KeyCode::Null).unwrap();
                last_tick = Instant::now();
            }
        }
    });

    // Main application loop
    loop {
        // Draw the UI
        terminal.draw(|f| {
            // Create layout with three sections: title, message list, and details
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3), // Title
                        Constraint::Min(0),    // Message list
                        Constraint::Length(3), // Details
                    ]
                    .as_ref(),
                )
                .split(f.size());

            // Render title
            let title = Paragraph::new("IMCP Protocol Analyzer")
                .style(Style::default().fg(Color::LightCyan))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(title, chunks[0]);

            // Render message list
            let items: Vec<ListItem> = app
                .messages
                .iter()
                .map(|record| {
                    let style = match record.direction {
                        MessageDirection::Incoming => Style::default().fg(Color::Green),
                        MessageDirection::Outgoing => Style::default().fg(Color::Yellow),
                    };
                    ListItem::new(Spans::from(vec![Span::styled(
                        app.format_message(record),
                        style,
                    )]))
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Messages"))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">> ");
            f.render_stateful_widget(list, chunks[1], &mut app.selected);

            // Render message details
            let details = if let Some(selected) = app.selected {
                if let Some(record) = app.messages.get(selected) {
                    Paragraph::new(app.format_message_details(record))
                        .block(Block::default().borders(Borders::ALL).title("Details"))
                } else {
                    Paragraph::new("No message selected")
                        .block(Block::default().borders(Borders::ALL).title("Details"))
                }
            } else {
                Paragraph::new("No message selected")
                    .block(Block::default().borders(Borders::ALL).title("Details"))
            };
            f.render_widget(details, chunks[2]);
        })?;

        // Handle user input
        if let Ok(key) = input_rx.try_recv() {
            match key {
                event::KeyCode::Char('q') => {
                    app.should_quit = true;
                }
                event::KeyCode::Up => {
                    if let Some(selected) = app.selected {
                        if selected > 0 {
                            app.selected = Some(selected - 1);
                        }
                    } else if !app.messages.is_empty() {
                        app.selected = Some(app.messages.len() - 1);
                    }
                }
                event::KeyCode::Down => {
                    if let Some(selected) = app.selected {
                        if selected < app.messages.len() - 1 {
                            app.selected = Some(selected + 1);
                        }
                    } else if !app.messages.is_empty() {
                        app.selected = Some(0);
                    }
                }
                _ => {}
            }
        }

        // Handle incoming messages
        while let Ok((message, is_incoming)) = message_rx.try_recv() {
            app.add_message(
                message,
                if is_incoming {
                    MessageDirection::Incoming
                } else {
                    MessageDirection::Outgoing
                },
            );
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
