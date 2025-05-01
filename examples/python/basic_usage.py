"""
IMCP (Interactive Message Control Protocol) Basic Usage Example

This example demonstrates the basic usage of IMCP client and server implementation,
showing how to establish a WebSocket connection, send data and control messages,
and handle responses.
"""

import asyncio
import os
from loguru import logger
from imcp.client import IMCPClient
from imcp.server import IMCPServer
from imcp.protocol import IMCPProtocol

async def run_server():
    """
    Run the IMCP server instance.
    
    This function initializes and starts an IMCP server that listens for incoming
    WebSocket connections on localhost:8765.
    """
    server = IMCPServer(host="localhost", port=8765)
    logger.info("Starting IMCP server...")
    await server.start()

async def run_client():
    """
    Run the IMCP client and demonstrate basic operations.
    
    This function:
    1. Connects to the IMCP server
    2. Sends a data message
    3. Receives and processes the response
    4. Sends a control message
    5. Receives and processes the control response
    6. Handles any errors and ensures proper cleanup
    """
    # Initialize client with server WebSocket URL
    client = IMCPClient("ws://localhost:8765")
    try:
        # Establish WebSocket connection to server
        await client.connect()
        logger.info("Connected to server")

        # Send a binary data message to server
        data = b"Hello, IMCP Server!"
        await client.send(data)
        logger.info(f"Sent data: {data.decode()}")

        # Wait for and receive response from server
        response = await client.receive()
        logger.info(f"Received response: {response}")

        # Send a control message with custom command and parameters
        await client.send_control("TEST", {"param1": "value1"})
        logger.info("Sent control message")

        # Wait for and receive control response from server
        control_response = await client.receive()
        logger.info(f"Received control response: {control_response}")

    except Exception as e:
        logger.error(f"Client error: {e}")
    finally:
        # Ensure proper cleanup of client connection
        await client.close()
        logger.info("Connection closed")

async def main():
    """
    Main function that orchestrates the example execution.
    
    This function:
    1. Starts the IMCP server in the background
    2. Waits for server initialization
    3. Runs the client operations
    4. Properly shuts down the server
    """
    # Start server as a background task
    server_task = asyncio.create_task(run_server())
    
    # Give server time to initialize
    await asyncio.sleep(1)
    
    # Run client operations
    await run_client()
    
    # Clean up server task
    server_task.cancel()
    try:
        await server_task
    except asyncio.CancelledError:
        pass

if __name__ == "__main__":
    # Configure logging to write to file with rotation
    logger.add("imcp_example.log", rotation="1 MB", level="INFO")
    
    # Run the example using asyncio event loop
    asyncio.run(main()) 