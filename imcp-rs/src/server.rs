//! IMCP server implementation
//!
//! This module provides the server-side functionality for the IMCP protocol.
//! Key features include:
//! - WebSocket server for client connections
//! - Secure handshake and session establishment
//! - Message handling and routing
//! - Client connection management
//! - Protocol security enforcement
//!
//! # Server Architecture
//! - Asynchronous WebSocket server using tokio
//! - Per-client connection handling in separate tasks
//! - Secure session management
//! - Message verification and processing
//!
//! # Security Features
//! - Secure handshake with public key exchange
//! - Message signature verification
//! - Session key establishment
//! - Client authentication
//!
//! # Usage Example
//! ```rust
//! use imcp::server::IMCPServer;
//! use imcp::protocol::IMCPHandler;
//!
//! // Create a custom message handler
//! struct MyHandler;
//! impl IMCPHandler for MyHandler {
//!     async fn handle_message(&self, message: Message, client_key: &[u8]) -> Result<Message, IMCPError> {
//!         // Handle messages here
//!         Ok(message)
//!     }
//! }
//!
//! // Create and start the server
//! let handler = Arc::new(MyHandler);
//! let mut server = IMCPServer::new(handler)?;
//! server.start("127.0.0.1", 8080).await?;
//! ```

use crate::protocol::{IMCPError, IMCPHandler, IMCPProtocol, Message};
use futures::{SinkExt, StreamExt};
use log::{debug, error, info};
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio_tungstenite::{accept_async, MaybeTlsStream, WebSocketStream};

/// IMCP server implementation for protocol communication
///
/// Manages server-side protocol operations including:
/// - WebSocket server handling
/// - Client connection management
/// - Message routing and processing
/// - Security enforcement
pub struct IMCPServer {
    /// Protocol instance for message handling
    /// Manages cryptographic operations and message formatting
    protocol: Arc<Mutex<IMCPProtocol>>,
    /// Message handler for processing client messages
    /// Implements custom business logic
    handler: Arc<dyn IMCPHandler + Send + Sync>,
    /// Active client connections
    /// Tracks connected WebSocket streams
    clients: HashSet<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl IMCPServer {
    /// Creates a new IMCP server instance
    ///
    /// Initializes the server with:
    /// - A new protocol instance
    /// - Custom message handler
    /// - Empty client set
    ///
    /// # Arguments
    /// * `handler` - Message handler implementing IMCPHandler trait
    ///
    /// # Returns
    /// * `Result<Self, IMCPError>` - New server instance or error
    pub fn new(handler: Arc<dyn IMCPHandler + Send + Sync>) -> Result<Self, IMCPError> {
        Ok(Self {
            protocol: Arc::new(Mutex::new(IMCPProtocol::new()?)),
            handler,
            clients: HashSet::new(),
        })
    }

    /// Starts the IMCP server
    ///
    /// Performs the following steps:
    /// 1. Binds to specified host and port
    /// 2. Accepts incoming connections
    /// 3. Spawns new tasks for client handling
    ///
    /// # Arguments
    /// * `host` - Server host address
    /// * `port` - Server port number
    ///
    /// # Returns
    /// * `Result<(), IMCPError>` - Result indicating success or failure
    pub async fn start(&mut self, host: &str, port: u16) -> Result<(), IMCPError> {
        let listener = TcpListener::bind(format!("{}:{}", host, port))
            .await
            .map_err(|e| IMCPError::ConnectionError(e.to_string()))?;

        info!("IMCP server started on {}:{}", host, port);

        while let Ok((stream, _)) = listener.accept().await {
            let protocol = self.protocol.clone();
            let handler = self.handler.clone();

            tokio::spawn(async move {
                if let Err(e) = Self::handle_client(stream, protocol, handler).await {
                    error!("Error handling client: {}", e);
                }
            });
        }

        Ok(())
    }

    /// Handles a client connection
    ///
    /// Manages the client connection lifecycle:
    /// 1. Accepts WebSocket connection
    /// 2. Processes client handshake
    /// 3. Sends server handshake
    /// 4. Establishes secure session
    /// 5. Processes client messages
    ///
    /// # Arguments
    /// * `stream` - TCP stream for the client connection
    /// * `protocol` - Protocol instance for message handling
    /// * `handler` - Message handler for processing
    ///
    /// # Returns
    /// * `Result<(), IMCPError>` - Result indicating success or failure
    async fn handle_client(
        stream: TcpStream,
        protocol: Arc<Mutex<IMCPProtocol>>,
        handler: Arc<dyn IMCPHandler + Send + Sync>,
    ) -> Result<(), IMCPError> {
        let ws_stream = accept_async(stream)
            .await
            .map_err(|e| IMCPError::ConnectionError(e.to_string()))?;

        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        // Receive client handshake
        if let Some(msg) = ws_receiver.next().await {
            let msg = msg.map_err(|e| IMCPError::ConnectionError(e.to_string()))?;
            let client_handshake: Message =
                bincode::deserialize(&msg.into_data()).map_err(|_| IMCPError::InvalidMessage)?;

            let protocol_guard = protocol.lock().await;
            if !protocol_guard.verify_message(&client_handshake, &client_handshake.body.data)? {
                return Err(IMCPError::InvalidSignature);
            }

            // Send server handshake
            let mut handshake = protocol_guard.create_handshake_message();
            protocol_guard.sign_message(&mut handshake)?;
            drop(protocol_guard);

            let handshake_bytes = bincode::serialize(&handshake)
                .map_err(|_| IMCPError::ProtocolError("Failed to serialize handshake".into()))?;

            ws_sender
                .send(handshake_bytes.into())
                .await
                .map_err(|e| IMCPError::ConnectionError(e.to_string()))?;

            // Establish secure connection
            let mut protocol_guard = protocol.lock().await;
            let session_key = protocol_guard.establish_connection(&client_handshake.body.data)?;
            drop(protocol_guard);
            info!("Secure connection established with client");

            // Handle messages
            while let Some(msg) = ws_receiver.next().await {
                let msg = msg.map_err(|e| IMCPError::ConnectionError(e.to_string()))?;
                let message: Message = bincode::deserialize(&msg.into_data())
                    .map_err(|_| IMCPError::InvalidMessage)?;

                let protocol_guard = protocol.lock().await;
                if !protocol_guard.verify_message(&message, &client_handshake.body.data)? {
                    return Err(IMCPError::InvalidSignature);
                }
                drop(protocol_guard);

                let response = handler
                    .handle_message(message, &client_handshake.body.data)
                    .await?;
                let response_bytes = bincode::serialize(&response)
                    .map_err(|_| IMCPError::ProtocolError("Failed to serialize response".into()))?;

                ws_sender
                    .send(response_bytes.into())
                    .await
                    .map_err(|e| IMCPError::ConnectionError(e.to_string()))?;
            }
        }

        Ok(())
    }
}
