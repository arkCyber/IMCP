"""
Secure Communication Example for IMCP Protocol

This module demonstrates secure communication between an IMCP server and client
using public-key cryptography for authentication and encryption.
"""

import asyncio
import os
from nacl.signing import SigningKey  # PyNaCl library for cryptographic operations
from loguru import logger  # Advanced logging library
from imcp.client import IMCPClient
from imcp.server import IMCPServer

async def run_secure_server():
    """
    Initialize and run a secure IMCP server with cryptographic key pair.
    
    This function:
    1. Generates a new Ed25519 signing key pair for the server
    2. Creates an IMCP server instance with the private key
    3. Starts the server and listens for secure connections
    """
    # Generate server's Ed25519 key pair for secure communication
    server_private_key = SigningKey.generate()
    logger.info("Generating server key pair...")
    
    # Initialize IMCP server with secure configuration
    server = IMCPServer(
        host="localhost",  # Server hostname
        port=8766,         # Server port
        private_key=server_private_key.encode()  # Server's private key for secure communication
    )
    logger.info("Starting secure IMCP server...")
    await server.start()

async def run_secure_client():
    """
    Initialize and run a secure IMCP client with cryptographic key pair.
    
    This function demonstrates:
    1. Client key pair generation
    2. Secure connection establishment
    3. Encrypted message exchange
    4. Secure control command handling
    """
    # Generate client's Ed25519 key pair for secure communication
    client_private_key = SigningKey.generate()
    logger.info("Generating client key pair...")
    
    # Initialize IMCP client with secure configuration
    client = IMCPClient(
        "ws://localhost:8766",  # WebSocket server URL
        private_key=client_private_key.encode()  # Client's private key for secure communication
    )
    
    try:
        # Establish secure WebSocket connection to server
        await client.connect()
        logger.info("Secure connection established")

        # Example: Send encrypted sensitive data
        sensitive_data = b"Confidential information"
        await client.send(sensitive_data)
        logger.info("Encrypted message sent")

        # Receive and decrypt response from server
        response = await client.receive()
        logger.info("Encrypted response received")

        # Example: Send secure control command with specific operation
        await client.send_control(
            "SECURE_OPERATION",  # Control command type
            {
                "operation": "encrypt",      # Operation to perform
                "data": "sensitive payload", # Data to process
                "algorithm": "AES-256"       # Encryption algorithm to use
            }
        )
        logger.info("Secure control command processed")

        # Receive secure response for the control command
        secure_response = await client.receive()
        logger.info("Secure response received")

    except Exception as e:
        logger.error(f"Secure client error: {e}")
    finally:
        # Ensure proper cleanup of client resources
        await client.close()

async def main():
    """
    Main function to orchestrate the secure communication example.
    
    This function:
    1. Starts the secure server in the background
    2. Waits for server initialization
    3. Runs the secure client
    4. Properly shuts down the server
    """
    # Start secure server as a background task
    server_task = asyncio.create_task(run_secure_server())
    
    # Allow time for server initialization
    await asyncio.sleep(1)
    
    # Run the secure client
    await run_secure_client()
    
    # Cleanup: Cancel and wait for server task
    server_task.cancel()
    try:
        await server_task
    except asyncio.CancelledError:
        pass

if __name__ == "__main__":
    # Configure logging with rotation and level settings
    logger.add(
        "imcp_secure_example.log",  # Log file name
        rotation="1 MB",            # Rotate log file when it reaches 1MB
        level="INFO"                # Log level
    )
    
    # Run the secure communication example
    asyncio.run(main()) 