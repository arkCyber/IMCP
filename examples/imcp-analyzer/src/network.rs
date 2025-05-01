//! Network module for handling WebSocket connections and IMCP message processing
//!
//! This module provides functionality to listen for WebSocket connections,
//! accept incoming connections, and process IMCP messages.

use anyhow::{Context, Result};
use imcp::protocol::Message;
use serde_json::from_slice;
use std::{
    net::{SocketAddr, TcpListener},
    sync::mpsc::Sender,
    thread,
};
use tungstenite::{accept, Message as WsMessage};

/// Starts a WebSocket listener on the specified port
///
/// # Arguments
/// * `port` - The port number to listen on
/// * `tx` - Channel sender for forwarding received messages to the main thread
///
/// # Returns
/// * `Result<()>` - Ok if the listener started successfully, Err otherwise
pub fn start_listener(port: u16, tx: Sender<(Message, bool)>) -> Result<()> {
    // Bind to localhost on the specified port
    let addr = format!("127.0.0.1:{}", port);
    let listener =
        TcpListener::bind(&addr).with_context(|| format!("Failed to bind to {}", addr))?;

    println!("Listening for IMCP messages on {}", addr);

    // Accept incoming connections
    for stream in listener.incoming() {
        let stream = stream?;
        let tx = tx.clone();

        // Spawn a new thread for each connection
        thread::spawn(move || {
            if let Err(e) = handle_connection(stream, tx) {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }

    Ok(())
}

/// Handles an individual WebSocket connection
///
/// # Arguments
/// * `stream` - The TCP stream for the connection
/// * `tx` - Channel sender for forwarding received messages
///
/// # Returns
/// * `Result<()>` - Ok if the connection was handled successfully, Err otherwise
fn handle_connection(stream: std::net::TcpStream, tx: Sender<(Message, bool)>) -> Result<()> {
    // Accept the WebSocket connection
    let mut ws_stream = accept(stream)?;
    let peer_addr = ws_stream.get_ref().peer_addr()?;

    println!("New connection from {}", peer_addr);

    // Process messages from the connection
    loop {
        match ws_stream.read_message()? {
            // Handle binary messages (IMCP messages)
            WsMessage::Binary(data) => {
                if let Ok(message) = from_slice::<Message>(&data) {
                    tx.send((message, true))?;
                }
            }
            // Handle connection close
            WsMessage::Close(_) => break,
            // Ignore other message types
            _ => continue,
        }
    }

    Ok(())
}
