// This example demonstrates the basic usage of the IMCP (Inter-Module Communication Protocol) library
// It shows how to set up a server and client, and how to exchange messages between them

// Import necessary modules from the IMCP library
use imcp::client::IMCPClient; // Client implementation for connecting to IMCP servers
use imcp::protocol::{IMCPError, IMCPHandler, IMCPProtocol, Message}; // Core protocol types and traits
use imcp::server::IMCPServer; // Server implementation for handling IMCP connections
use log::{error, info}; // Logging utilities
use std::collections::HashMap; // For storing control message parameters
use std::sync::Arc; // For thread-safe reference counting
use tokio; // Async runtime

// Define a basic message handler that implements the IMCPHandler trait
// This handler will process incoming messages and generate appropriate responses
struct BasicHandler;

// Implement the IMCPHandler trait for our BasicHandler
#[async_trait::async_trait]
impl IMCPHandler for BasicHandler {
    // Handle incoming messages from clients
    // This method is called whenever a message is received by the server
    async fn handle_message(
        &self,
        message: Message,          // The incoming message
        _client_public_key: &[u8], // Client's public key (not used in this basic example)
    ) -> Result<Message, IMCPError> {
        // Process different types of messages
        match message.header.message_type {
            // Handle data messages - simply echo back the received data
            imcp::protocol::MessageType::Data => {
                let response = Message {
                    header: message.header, // Keep the same header
                    body: message.body,     // Echo back the body
                    signature: Vec::new(),  // No signature in this basic example
                };
                Ok(response)
            }
            // Handle control messages - echo back the control parameters
            imcp::protocol::MessageType::Control => {
                let response = Message {
                    header: message.header, // Keep the same header
                    body: message.body,     // Echo back the body
                    signature: Vec::new(),  // No signature in this basic example
                };
                Ok(response)
            }
            // Return error for unsupported message types
            _ => Err(IMCPError::ProtocolError("Unsupported message type".into())),
        }
    }
}

// Main function demonstrating the complete IMCP workflow
#[tokio::main]
async fn main() -> Result<(), IMCPError> {
    // Initialize logging system
    env_logger::init();

    // Create and start the IMCP server
    let handler = Arc::new(BasicHandler); // Wrap handler in Arc for thread safety
    let mut server = IMCPServer::new(handler)?; // Create new server instance
    let server_handle = tokio::spawn(async move {
        // Start server on localhost:8765
        if let Err(e) = server.start("localhost", 8765).await {
            error!("Server error: {}", e);
        }
    });

    // Give the server time to start up
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Create and connect IMCP client
    let mut client = IMCPClient::new()?; // Create new client instance
    client.connect("ws://localhost:8765").await?; // Connect to server

    // Example 1: Send and receive a data message
    let data = b"Hello, IMCP Server!".to_vec(); // Create test data
    client.send(data).await?; // Send data to server

    let response = client.receive().await?; // Receive server's response
    info!("Received response: {:?}", response); // Log the response

    // Example 2: Send and receive a control message
    let mut params = HashMap::new(); // Create control parameters
    params.insert("param1".to_string(), "value1".to_string()); // Add a parameter
    client.send_control("TEST", params).await?; // Send control message

    let control_response = client.receive().await?; // Receive control response
    info!("Received control response: {:?}", control_response); // Log the response

    // Clean up: close client connection and stop server
    client.close().await?; // Close client connection
    server_handle.abort(); // Stop the server

    Ok(())
}
