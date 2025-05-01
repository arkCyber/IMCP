//! Industrial Control System Example
//! This example demonstrates a simple industrial control system using IMCP protocol
//! It simulates monitoring and controlling industrial devices with metrics like temperature and pressure

use imcp::client::IMCPClient;
use imcp::protocol::{IMCPError, IMCPHandler, IMCPProtocol, Message};
use imcp::server::IMCPServer;
use log::{debug, error, info};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio;

/// Represents an industrial device with various metrics and status
struct IndustrialDevice {
    device_id: String,            // Unique identifier for the device
    temperature: f64,             // Current temperature reading
    pressure: f64,                // Current pressure reading
    status: String,               // Current operational status
    last_maintenance: SystemTime, // Timestamp of last maintenance
}

impl IndustrialDevice {
    /// Creates a new industrial device with default values
    fn new(device_id: String) -> Self {
        Self {
            device_id,
            temperature: 25.0, // Default temperature in Celsius
            pressure: 1.0,     // Default pressure in bar
            status: "IDLE".to_string(),
            last_maintenance: SystemTime::now(),
        }
    }

    /// Updates device metrics with random variations
    /// Simulates real-world sensor readings with controlled randomness
    fn update_metrics(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        // Simulate small random variations in temperature and pressure
        self.temperature += rng.gen_range(-0.5..0.5);
        self.pressure += rng.gen_range(-0.1..0.1);

        // Ensure metrics stay within safe operating ranges
        self.temperature = self.temperature.max(20.0).min(30.0);
        self.pressure = self.pressure.max(0.5).min(2.0);

        debug!("Device {} metrics updated", self.device_id);
    }

    /// Returns all device metrics as a HashMap
    /// Used for reporting device status to monitoring systems
    fn get_metrics(&self) -> HashMap<String, String> {
        let mut metrics = HashMap::new();
        metrics.insert("device_id".to_string(), self.device_id.clone());
        metrics.insert("temperature".to_string(), self.temperature.to_string());
        metrics.insert("pressure".to_string(), self.pressure.to_string());
        metrics.insert("status".to_string(), self.status.clone());
        metrics.insert(
            "last_maintenance".to_string(),
            self.last_maintenance
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string(),
        );
        metrics
    }
}

/// Handles IMCP protocol messages for industrial devices
struct IndustrialHandler {
    devices: HashMap<String, IndustrialDevice>, // Map of device IDs to device instances
}

impl IndustrialHandler {
    /// Creates a new handler with three simulated devices
    fn new() -> Self {
        let mut devices = HashMap::new();
        for i in 1..=3 {
            let device_id = format!("DEV{:03}", i);
            devices.insert(device_id.clone(), IndustrialDevice::new(device_id));
        }
        Self { devices }
    }
}

#[async_trait::async_trait]
impl IMCPHandler for IndustrialHandler {
    /// Handles incoming IMCP messages and returns appropriate responses
    async fn handle_message(
        &self,
        message: Message,
        _client_public_key: &[u8],
    ) -> Result<Message, IMCPError> {
        match message.header.message_type {
            // Handle data requests for device metrics
            imcp::protocol::MessageType::Data => {
                let device_id = String::from_utf8(message.body.data.clone())
                    .map_err(|_| IMCPError::ProtocolError("Invalid device ID".into()))?;

                if let Some(device) = self.devices.get(&device_id) {
                    let metrics = device.get_metrics();
                    let response = Message {
                        header: message.header,
                        body: imcp::protocol::MessageBody {
                            data: serde_json::to_vec(&metrics).map_err(|_| {
                                IMCPError::ProtocolError("Failed to serialize metrics".into())
                            })?,
                            control_type: None,
                            parameters: None,
                        },
                        signature: Vec::new(),
                    };
                    Ok(response)
                } else {
                    Err(IMCPError::ProtocolError("Device not found".into()))
                }
            }
            // Handle control commands for devices
            imcp::protocol::MessageType::Control => {
                let params = message.body.parameters.unwrap_or_default();
                let device_id = params
                    .get("device_id")
                    .ok_or_else(|| IMCPError::ProtocolError("Missing device ID".into()))?;
                let command = params
                    .get("command")
                    .ok_or_else(|| IMCPError::ProtocolError("Missing command".into()))?;

                if let Some(device) = self.devices.get(device_id) {
                    let mut response_params = HashMap::new();
                    response_params.insert("status".to_string(), "success".to_string());
                    response_params.insert("device_status".to_string(), device.status.clone());

                    let response = Message {
                        header: message.header,
                        body: imcp::protocol::MessageBody {
                            data: Vec::new(),
                            control_type: Some("RESPONSE".to_string()),
                            parameters: Some(response_params),
                        },
                        signature: Vec::new(),
                    };
                    Ok(response)
                } else {
                    Err(IMCPError::ProtocolError("Device not found".into()))
                }
            }
            _ => Err(IMCPError::ProtocolError("Unsupported message type".into())),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), IMCPError> {
    // Initialize logging system
    env_logger::init();

    // Start industrial server with handler
    let handler = Arc::new(IndustrialHandler::new());
    let mut server = IMCPServer::new(handler)?;
    let server_handle = tokio::spawn(async move {
        if let Err(e) = server.start("localhost", 8767).await {
            error!("Server error: {}", e);
        }
    });

    // Wait for server to start up
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Initialize and connect client
    let mut client = IMCPClient::new()?;
    client.connect("ws://localhost:8767").await?;

    // Monitor all devices
    for device_id in ["DEV001", "DEV002", "DEV003"] {
        // Request current metrics for each device
        let mut params = HashMap::new();
        params.insert("device_id".to_string(), device_id.to_string());
        client.send_control("GET_METRICS", params).await?;

        let metrics = client.receive().await?;
        info!("Device {} metrics: {:?}", device_id, metrics);

        // Implement safety control: stop device if temperature is too high
        if let Some(params) = metrics.body.parameters {
            if let Some(temp_str) = params.get("temperature") {
                if let Ok(temp) = temp_str.parse::<f64>() {
                    if temp > 28.0 {
                        let mut control_params = HashMap::new();
                        control_params.insert("device_id".to_string(), device_id.to_string());
                        control_params.insert("command".to_string(), "STOP".to_string());
                        client.send_control("CONTROL", control_params).await?;

                        let response = client.receive().await?;
                        info!("Device {} control response: {:?}", device_id, response);
                    }
                }
            }
        }
    }

    // Schedule maintenance for a specific device
    let mut maintenance_params = HashMap::new();
    maintenance_params.insert("device_id".to_string(), "DEV001".to_string());
    maintenance_params.insert("command".to_string(), "MAINTENANCE".to_string());
    client.send_control("CONTROL", maintenance_params).await?;

    let response = client.receive().await?;
    info!("Maintenance scheduled for DEV001");

    // Clean up: close client connection and stop server
    client.close().await?;
    server_handle.abort();

    Ok(())
}
