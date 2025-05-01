# IMCP Protocol Examples

This directory contains example implementations of the IMCP (Industrial Model Context Protocol) in different programming languages.

## Language-Specific Examples

- [Python Examples](python/README.md) - Python implementation examples
- [Rust Examples](rust/README.md) - Rust implementation examples

## Common Features

All examples demonstrate:
1. **Connection Management**
   - Secure connection establishment
   - Connection state tracking
   - Graceful connection closure

2. **Message Handling**
   - Data message processing
   - Control message processing
   - Error message handling

3. **Security Features**
   - Message encryption
   - Message signing
   - Key management

4. **Logging**
   - Detailed operation logs
   - Error tracking
   - Performance metrics

## Troubleshooting

If you encounter issues:

1. **Connection Problems**:
   - Check if the required ports (8765, 8766, 8767) are available
   - Verify network connectivity
   - Check firewall settings

2. **Dependency Issues**:
   - Ensure all required packages/crates are installed
   - Check language version compatibility
   - Verify environment setup

3. **Log Analysis**:
   - Check the relevant log file for error messages
   - Look for connection timeouts or authentication failures
   - Monitor for resource usage warnings

## Setup Instructions

Before running any examples, ensure you have the required environment:

1. Install Python 3.8 or higher
2. Install required packages:
   ```bash
   pip install -r requirements.txt
   ```
3. Create a virtual environment (recommended):
   ```bash
   python -m venv venv
   source venv/bin/activate  # On Windows: venv\Scripts\activate
   ```

## Basic Usage Example

The `basic_usage.py` example demonstrates the fundamental usage of IMCP protocol for client-server communication.

### Features Demonstrated:
- Basic client-server connection establishment
- Data message exchange
- Control message handling
- Error handling and logging

### Step-by-Step Operation:

1. **Start the Example**:
   ```bash
   python basic_usage.py
   ```

2. **Observe the Process**:
   - The server starts on localhost:8765
   - The client connects to the server
   - A handshake process is completed
   - Data messages are exchanged
   - Control messages are processed
   - The connection is closed

3. **Check Logs**:
   ```bash
   cat imcp_example.log
   ```

### Expected Output:
```
INFO: Starting IMCP server...
INFO: Connected to server
INFO: Sent data: Hello, IMCP Server!
INFO: Received response: {...}
INFO: Sent control message
INFO: Received control response: {...}
INFO: Connection closed
```

## Secure Communication Example

The `secure_communication.py` example showcases the security features of IMCP protocol.

### Features Demonstrated:
- Key pair generation and management
- Secure handshake process
- Encrypted data transmission
- Secure control command handling
- Message signing and verification

### Step-by-Step Operation:

1. **Start the Example**:
   ```bash
   python secure_communication.py
   ```

2. **Key Generation Process**:
   - Server generates its key pair
   - Client generates its key pair
   - Keys are exchanged securely

3. **Secure Communication**:
   - Handshake establishes secure session
   - Messages are encrypted and signed
   - Control commands are processed securely

4. **Check Logs**:
   ```bash
   cat imcp_secure_example.log
   ```

### Expected Output:
```
INFO: Generating server key pair...
INFO: Generating client key pair...
INFO: Secure connection established
INFO: Encrypted message sent
INFO: Encrypted response received
INFO: Secure control command processed
```

## Industrial Control Example

The `industrial_control.py` example demonstrates the application of IMCP protocol in industrial control systems.

### Features Demonstrated:
- Industrial device simulation
- Real-time metrics monitoring
- Device control commands
- Maintenance scheduling
- Error handling in industrial context

### Step-by-Step Operation:

1. **Start the Example**:
   ```bash
   python industrial_control.py
   ```

2. **Device Simulation**:
   - Three simulated devices are created (DEV001, DEV002, DEV003)
   - Each device has temperature and pressure sensors
   - Devices update their metrics periodically

3. **Monitoring Process**:
   - Client connects to server
   - Requests metrics from each device
   - Monitors for abnormal conditions
   - Sends control commands when needed

4. **Maintenance Operations**:
   - Schedule maintenance for devices
   - Monitor maintenance status
   - Track maintenance history

5. **Check Logs**:
   ```bash
   cat imcp_industrial_example.log
   ```

### Expected Output:
```
INFO: Starting industrial control server...
INFO: Simulated devices initialized
INFO: Connected to industrial control server
INFO: Device DEV001 metrics: {...}
INFO: Device DEV002 metrics: {...}
INFO: Device DEV003 metrics: {...}
INFO: Maintenance scheduled for DEV001
```

## Advanced Usage

For more advanced scenarios:

1. **Multiple Clients**:
   - Modify the examples to handle multiple concurrent clients
   - Implement client management and load balancing

2. **Custom Device Types**:
   - Extend the industrial example with custom device types
   - Add specific control commands and metrics

3. **Enhanced Security**:
   - Implement custom key management
   - Add additional security layers
   - Configure specific encryption algorithms

## Log Files

Each example generates its own log file:
- `imcp_example.log` - Basic usage example logs
- `imcp_secure_example.log` - Secure communication example logs
- `imcp_industrial_example.log` - Industrial control example logs

## Requirements

All examples require the following Python packages:
- asyncio
- websockets
- pynacl
- msgpack
- loguru

These dependencies are listed in the project's `requirements.txt` file.

## Notes

1. All examples use localhost for demonstration purposes. For production use, replace with appropriate host addresses.
2. The examples use different ports (8765, 8766, 8767) to avoid conflicts when running multiple examples.
3. The industrial control example simulates device behavior. In a real implementation, this would be replaced with actual device communication.
4. Security keys in the examples are generated for demonstration. In production, proper key management should be implemented. 