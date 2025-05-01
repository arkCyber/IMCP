// Import necessary modules from the IMCP (Inter-Module Communication Protocol) crate
use imcp::client::IMCPClient;
use imcp::protocol::{IMCPError, IMCPHandler, IMCPProtocol, Message};
use imcp::server::IMCPServer;
use log::{error, info};
use std::collections::HashMap;
use std::sync::Arc;
use tokio;

// Define a handler struct that will process incoming messages
// This struct implements the IMCPHandler trait to handle different types of messages
struct SecureHandler;

// Implement the IMCPHandler trait for SecureHandler
// This implementation defines how the handler processes different types of messages
#[async_trait::async_trait]
impl IMCPHandler for SecureHandler {
    // Handle incoming messages based on their type
    // Parameters:
    // - message: The incoming message to process
    // - client_public_key: The public key of the client for verification
    // Returns: Result containing either a response message or an error
    async fn handle_message(
        &self,
        message: Message,
        _client_public_key: &[u8],
    ) -> Result<Message, IMCPError> {
        match message.header.message_type {
            // Handle data messages by creating a response with the same header and body
            imcp::protocol::MessageType::Data => {
                let response = Message {
                    header: message.header,
                    body: message.body,
                    signature: Vec::new(),
                };
                Ok(response)
            }
            // Handle control messages by creating a response with the same header and body
            imcp::protocol::MessageType::Control => {
                let response = Message {
                    header: message.header,
                    body: message.body,
                    signature: Vec::new(),
                };
                Ok(response)
            }
            // Return an error for any unsupported message types
            _ => Err(IMCPError::ProtocolError("Unsupported message type".into())),
        }
    }
}

// Main function that demonstrates secure communication between client and server
#[tokio::main]
async fn main() -> Result<(), IMCPError> {
    // Initialize logging system
    env_logger::init();

    // Create and start the secure server
    // Wrap the handler in an Arc for thread-safe sharing
    let handler = Arc::new(SecureHandler);
    let mut server = IMCPServer::new(handler)?;

    // Spawn the server in a separate task
    let server_handle = tokio::spawn(async move {
        if let Err(e) = server.start("localhost", 8766).await {
            error!("Server error: {}", e);
        }
    });

    // Wait for the server to initialize
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Create and connect the client to the server
    let mut client = IMCPClient::new()?;
    client.connect("ws://localhost:8766").await?;

    // Send sensitive data to the server
    let sensitive_data = b"Confidential information".to_vec();
    client.send(sensitive_data).await?;

    // Receive and log the encrypted response from the server
    let response = client.receive().await?;
    info!("Received encrypted response");

    // Send a secure control command with encryption parameters
    let mut params = HashMap::new();
    params.insert("operation".to_string(), "encrypt".to_string());
    params.insert("data".to_string(), "sensitive payload".to_string());
    params.insert("algorithm".to_string(), "AES-256".to_string());
    client.send_control("SECURE_OPERATION", params).await?;

    // Receive and log the secure response
    let secure_response = client.receive().await?;
    info!("Received secure response");

    // Close the client connection
    client.close().await?;

    // Stop the server
    server_handle.abort();

    Ok(())
}
