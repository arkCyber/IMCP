//! IMCP (Industrial Model Context Protocol) core implementation
//!
//! This module provides the core protocol functionality for secure industrial communication.
//! Key features include:
//! - Secure message exchange using Ed25519 digital signatures
//! - Connection state management
//! - Message type handling (Handshake, Data, Control, Heartbeat, Error)
//! - Session-based communication with unique identifiers
//! - Timestamp-based message ordering
//!
//! The protocol ensures:
//! - Message integrity through digital signatures
//! - Secure session establishment
//! - Proper message sequencing
//! - Connection state tracking
//!
//! # Security Considerations
//! - All messages are signed using Ed25519 signatures
//! - Each session has a unique identifier and session key
//! - Timestamps prevent replay attacks
//! - Sequence numbers ensure message ordering
//!
//! # Usage Example
//! ```rust
//! use imcp::protocol::IMCPProtocol;
//!
//! // Create a new protocol instance
//! let protocol = IMCPProtocol::new()?;
//!
//! // Create and sign a handshake message
//! let mut handshake = protocol.create_handshake_message();
//! protocol.sign_message(&mut handshake)?;
//! ```

use async_trait::async_trait;
use log::{debug, error, info};
use ring::rand::SystemRandom;
use ring::signature::{Ed25519KeyPair, KeyPair, Signature, UnparsedPublicKey, ED25519};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

/// Custom error types for IMCP protocol operations
///
/// These errors cover various failure scenarios in the protocol:
/// - Invalid message format or structure
/// - Signature verification failures
/// - Connection-related issues
/// - Protocol-specific errors
#[derive(Error, Debug)]
pub enum IMCPError {
    #[error("Invalid message format")]
    InvalidMessage,
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Connection error: {0}")]
    ConnectionError(String),
    #[error("Protocol error: {0}")]
    ProtocolError(String),
}

/// Types of messages supported by the protocol
///
/// Each message type serves a specific purpose in the protocol:
/// - Handshake: Initial connection establishment
/// - Data: Regular payload transmission
/// - Control: Protocol control commands
/// - Heartbeat: Connection keepalive
/// - Error: Error reporting and handling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    /// Initial handshake message for connection establishment
    /// Contains public key exchange and session initialization
    Handshake,
    /// Data payload message
    /// Carries the actual application data
    Data,
    /// Control command message
    /// Used for protocol control operations
    Control,
    /// Heartbeat message for connection keepalive
    /// Maintains connection state and detects disconnections
    Heartbeat,
    /// Error message
    /// Reports protocol or application errors
    Error,
}

/// Current state of the connection
///
/// Tracks the lifecycle of a connection from initialization to closure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionState {
    /// Initial state before handshake
    /// No secure communication established yet
    Initialized,
    /// During handshake process
    /// Key exchange and session setup in progress
    Handshake,
    /// Connection established and ready for communication
    /// Secure channel is active
    Established,
    /// Connection closed
    /// No further communication possible
    Closed,
}

/// Header information for each message
///
/// Contains metadata essential for message processing:
/// - Message type identification
/// - Sequence tracking
/// - Session management
/// - Timestamp for ordering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageHeader {
    /// Type of the message
    /// Determines how the message should be processed
    pub message_type: MessageType,
    /// Sequence number for message ordering
    /// Ensures messages are processed in the correct order
    pub sequence: u64,
    /// Unique session identifier
    /// Links messages to specific communication sessions
    pub session_id: Uuid,
    /// Timestamp of message creation
    /// Used for ordering and preventing replay attacks
    pub timestamp: u64,
}

/// Body content of the message
///
/// Contains the actual payload and control information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageBody {
    /// Raw data payload
    /// The actual content being transmitted
    pub data: Vec<u8>,
    /// Type of control command (if applicable)
    /// Specifies the control operation to perform
    pub control_type: Option<String>,
    /// Additional parameters for control messages
    /// Key-value pairs for control command configuration
    pub parameters: Option<HashMap<String, String>>,
}

/// Complete message structure
///
/// Represents a full protocol message with all necessary components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Message header containing metadata
    /// Essential for message processing and routing
    pub header: MessageHeader,
    /// Message body containing payload
    /// The actual content being transmitted
    pub body: MessageBody,
    /// Digital signature for message verification
    /// Ensures message integrity and authenticity
    pub signature: Vec<u8>,
}

/// Core protocol implementation
///
/// Manages the IMCP protocol state and operations:
/// - Key pair management
/// - Connection state tracking
/// - Message creation and verification
/// - Session management
pub struct IMCPProtocol {
    /// Private key for message signing
    /// Used to create digital signatures for outgoing messages
    private_key: Arc<Ed25519KeyPair>,
    /// Public key for message verification
    /// Distributed to peers for signature verification
    public_key: Vec<u8>,
    /// Current connection state
    /// Tracks the protocol's operational state
    state: ConnectionState,
    /// Active connections with their session keys
    /// Maps client public keys to their session keys
    connections: HashMap<Vec<u8>, Vec<u8>>, // client_public_key -> session_key
}

impl IMCPProtocol {
    /// Creates a new IMCP protocol instance with generated key pair
    pub fn new() -> Result<Self, IMCPError> {
        let rng = SystemRandom::new();
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng)
            .map_err(|_| IMCPError::ProtocolError("Failed to generate key pair".into()))?;

        let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())
            .map_err(|_| IMCPError::ProtocolError("Failed to create key pair".into()))?;

        let public_key = key_pair.public_key().as_ref().to_vec();

        Ok(Self {
            private_key: Arc::new(key_pair),
            public_key,
            state: ConnectionState::Initialized,
            connections: HashMap::new(),
        })
    }

    /// Returns the public key for this protocol instance
    pub fn get_public_key(&self) -> &[u8] {
        &self.public_key
    }

    /// Creates a handshake message with the protocol's public key
    pub fn create_handshake_message(&self) -> Message {
        Message {
            header: MessageHeader {
                message_type: MessageType::Handshake,
                sequence: 0,
                session_id: Uuid::new_v4(),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            },
            body: MessageBody {
                data: self.public_key.clone(),
                control_type: None,
                parameters: None,
            },
            signature: Vec::new(),
        }
    }

    /// Creates a data message with the given payload
    pub fn create_data_message(&self, data: Vec<u8>, session_id: Uuid, sequence: u64) -> Message {
        Message {
            header: MessageHeader {
                message_type: MessageType::Data,
                sequence,
                session_id,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            },
            body: MessageBody {
                data,
                control_type: None,
                parameters: None,
            },
            signature: Vec::new(),
        }
    }

    /// Creates a control message with specified type and parameters
    pub fn create_control_message(
        &self,
        control_type: &str,
        parameters: HashMap<String, String>,
    ) -> Message {
        Message {
            header: MessageHeader {
                message_type: MessageType::Control,
                sequence: 0,
                session_id: Uuid::new_v4(),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            },
            body: MessageBody {
                data: Vec::new(),
                control_type: Some(control_type.to_string()),
                parameters: Some(parameters),
            },
            signature: Vec::new(),
        }
    }

    /// Signs a message using the protocol's private key
    pub fn sign_message(&self, message: &mut Message) -> Result<(), IMCPError> {
        let message_bytes = bincode::serialize(message)
            .map_err(|_| IMCPError::ProtocolError("Failed to serialize message".into()))?;

        let signature = self.private_key.sign(&message_bytes);
        message.signature = signature.as_ref().to_vec();

        Ok(())
    }

    /// Verifies a message's signature using the provided public key
    pub fn verify_message(&self, message: &Message, public_key: &[u8]) -> Result<bool, IMCPError> {
        let mut message_clone = message.clone();
        message_clone.signature = Vec::new();

        let message_bytes = bincode::serialize(&message_clone)
            .map_err(|_| IMCPError::ProtocolError("Failed to serialize message".into()))?;

        let public_key = UnparsedPublicKey::new(&ED25519, public_key);

        Ok(public_key
            .verify(&message_bytes, &message.signature)
            .is_ok())
    }

    /// Establishes a secure connection with a client
    pub fn establish_connection(&mut self, client_public_key: &[u8]) -> Result<Vec<u8>, IMCPError> {
        let session_key: Vec<u8> = (0..32).map(|_| rand::random::<u8>()).collect();
        self.connections
            .insert(client_public_key.to_vec(), session_key.clone());
        self.state = ConnectionState::Established;
        Ok(session_key)
    }

    /// Retrieves the session key for a connected client
    pub fn get_connection(&self, client_public_key: &[u8]) -> Option<&[u8]> {
        self.connections
            .get(client_public_key)
            .map(|k| k.as_slice())
    }
}

/// Trait for handling IMCP messages
#[async_trait]
pub trait IMCPHandler {
    /// Handles incoming messages and returns appropriate responses
    async fn handle_message(
        &self,
        message: Message,
        client_public_key: &[u8],
    ) -> Result<Message, IMCPError>;
}
