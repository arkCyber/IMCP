# IMCP Protocol Python Examples

This directory contains Python example implementations demonstrating the usage of the IMCP (Industrial Model Context Protocol).

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

## Examples

### Basic Usage Example (`basic_usage.py`)
Demonstrates the fundamental usage of IMCP protocol for client-server communication.

### Secure Communication Example (`secure_communication.py`)
Showcases the security features of IMCP protocol including key pair generation, secure handshake, and encrypted communication.

### Industrial Control Example (`industrial_control.py`)
Demonstrates the application of IMCP protocol in industrial control systems with device simulation and monitoring.

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