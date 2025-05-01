//! IMCP client implementation
//!
//! This module provides the client-side functionality for the IMCP protocol.
//! Key features include:
//! - WebSocket-based communication with IMCP servers
//! - Secure handshake and session establishment
//! - Message sending and receiving
//! - Connection state management
//! - Sequence number tracking
//!
//! # Connection Lifecycle
//! 1. Client initialization with protocol instance
//! 2. WebSocket connection establishment
//! 3. Secure handshake with server
//! 4. Message exchange
//! 5. Connection closure
//!
//! # Security Features
//! - Secure handshake with public key exchange
//! - Message signing and verification
//! - Session-based communication
//! - Sequence number tracking for message ordering
//!
//! # Usage Example
//! ```rust
//! use imcp::client::IMCPClient;
//!
//! // Create a new client instance
//! let mut client = IMCPClient::new()?;
//!
//! // Connect to the server
//! client.connect("ws://localhost:8080").await?;
//!
//! // Send data
//! client.send(vec![1, 2, 3]).await?;
//!
//! // Receive response
//! let response = client.receive().await?;
//! ```

use crate::protocol::{IMCPError, IMCPProtocol, Message};
use futures::{SinkExt, StreamExt};
use log::{debug, error, info};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

/// IMCP client implementation for protocol communication
///
/// Manages client-side protocol operations including:
/// - WebSocket connection handling
/// - Message creation and sending
/// - Message reception and processing
/// - Session management
/// - Sequence number tracking
pub struct IMCPClient {
    /// Protocol instance for message handling
    /// Manages cryptographic operations and message formatting
    protocol: Arc<IMCPProtocol>,
    /// WebSocket connection to the server
    /// Handles the actual network communication
    websocket: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    /// Current session identifier
    /// Links messages to the active communication session
    session_id: Option<uuid::Uuid>,
    /// Message sequence number
    /// Ensures messages are processed in the correct order
    sequence: u64,
}

impl IMCPClient {
    /// Creates a new IMCP client instance
    ///
    /// Initializes the client with:
    /// - A new protocol instance
    /// - Empty WebSocket connection
    /// - No active session
    /// - Initial sequence number
    ///
    /// # Returns
    /// * `Result<Self, IMCPError>` - New client instance or error
    pub fn new() -> Result<Self, IMCPError> {
        Ok(Self {
            protocol: Arc::new(IMCPProtocol::new()?),
            websocket: None,
            session_id: None,
            sequence: 0,
        })
    }

    /// Establishes a connection to the IMCP server
    ///
    /// Performs the following steps:
    /// 1. Establishes WebSocket connection
    /// 2. Creates and signs handshake message
    /// 3. Sends handshake to server
    /// 4. Receives and verifies server handshake
    /// 5. Initializes session
    ///
    /// # Arguments
    /// * `url` - WebSocket URL of the server
    ///
    /// # Returns
    /// * `Result<(), IMCPError>` - Result indicating success or failure
    pub async fn connect(&mut self, url: &str) -> Result<(), IMCPError> {
        // Establish WebSocket connection
        let (ws_stream, _) = connect_async(url)
            .await
            .map_err(|e| IMCPError::ConnectionError(e.to_string()))?;

        self.websocket = Some(ws_stream);

        // Send handshake message
        let mut handshake = self.protocol.create_handshake_message();
        self.protocol.sign_message(&mut handshake)?;

        let handshake_bytes = bincode::serialize(&handshake)
            .map_err(|_| IMCPError::ProtocolError("Failed to serialize handshake".into()))?;

        if let Some(ws) = &mut self.websocket {
            ws.send(handshake_bytes.into())
                .await
                .map_err(|e| IMCPError::ConnectionError(e.to_string()))?;
        }

        // Receive and verify server handshake
        if let Some(ws) = &mut self.websocket {
            if let Some(msg) = ws.next().await {
                let msg = msg.map_err(|e| IMCPError::ConnectionError(e.to_string()))?;
                let server_handshake: Message = bincode::deserialize(&msg.into_data())
                    .map_err(|_| IMCPError::InvalidMessage)?;

                if !self
                    .protocol
                    .verify_message(&server_handshake, &server_handshake.body.data)?
                {
                    return Err(IMCPError::InvalidSignature);
                }

                self.session_id = Some(server_handshake.header.session_id);
                info!("Connected to server");
            }
        }

        Ok(())
    }

    /// Sends a data message to the server
    ///
    /// Creates and sends a data message with:
    /// - Current session ID
    /// - Incremented sequence number
    /// - Digital signature
    ///
    /// # Arguments
    /// * `data` - Data payload to send
    ///
    /// # Returns
    /// * `Result<(), IMCPError>` - Result indicating success or failure
    pub async fn send(&mut self, data: Vec<u8>) -> Result<(), IMCPError> {
        if let Some(session_id) = self.session_id {
            let mut message = self
                .protocol
                .create_data_message(data, session_id, self.sequence);
            self.protocol.sign_message(&mut message)?;

            let message_bytes = bincode::serialize(&message)
                .map_err(|_| IMCPError::ProtocolError("Failed to serialize message".into()))?;

            if let Some(ws) = &mut self.websocket {
                ws.send(message_bytes.into())
                    .await
                    .map_err(|e| IMCPError::ConnectionError(e.to_string()))?;
                self.sequence += 1;
                info!("Sent data message");
            }
        }
        Ok(())
    }

    /// Sends a control message to the server
    ///
    /// Creates and sends a control message with:
    /// - Specified control type
    /// - Additional parameters
    /// - Digital signature
    ///
    /// # Arguments
    /// * `control_type` - Type of control command
    /// * `parameters` - Additional parameters for the control command
    ///
    /// # Returns
    /// * `Result<(), IMCPError>` - Result indicating success or failure
    pub async fn send_control(
        &mut self,
        control_type: &str,
        parameters: HashMap<String, String>,
    ) -> Result<(), IMCPError> {
        let mut message = self
            .protocol
            .create_control_message(control_type, parameters);
        self.protocol.sign_message(&mut message)?;

        let message_bytes = bincode::serialize(&message)
            .map_err(|_| IMCPError::ProtocolError("Failed to serialize message".into()))?;

        if let Some(ws) = &mut self.websocket {
            ws.send(message_bytes.into())
                .await
                .map_err(|e| IMCPError::ConnectionError(e.to_string()))?;
            info!("Sent control message");
        }
        Ok(())
    }

    /// Receives a message from the server
    ///
    /// Handles incoming messages by:
    /// 1. Waiting for WebSocket message
    /// 2. Deserializing message data
    /// 3. Returning the message or error
    ///
    /// # Returns
    /// * `Result<Message, IMCPError>` - Received message or error
    pub async fn receive(&mut self) -> Result<Message, IMCPError> {
        if let Some(ws) = &mut self.websocket {
            if let Some(msg) = ws.next().await {
                let msg = msg.map_err(|e| IMCPError::ConnectionError(e.to_string()))?;
                let message: Message = bincode::deserialize(&msg.into_data())
                    .map_err(|_| IMCPError::InvalidMessage)?;
                info!("Received message");
                Ok(message)
            } else {
                Err(IMCPError::ConnectionError("Connection closed".into()))
            }
        } else {
            Err(IMCPError::ConnectionError("Not connected".into()))
        }
    }

    /// Closes the connection to the server
    ///
    /// Performs graceful connection closure by:
    /// 1. Sending close frame to server
    /// 2. Waiting for acknowledgment
    /// 3. Cleaning up resources
    ///
    /// # Returns
    /// * `Result<(), IMCPError>` - Result indicating success or failure
    pub async fn close(&mut self) -> Result<(), IMCPError> {
        if let Some(ws) = &mut self.websocket {
            ws.close(None)
                .await
                .map_err(|e| IMCPError::ConnectionError(e.to_string()))?;
            info!("Connection closed");
        }
        Ok(())
    }
}
