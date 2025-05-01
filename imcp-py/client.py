"""
IMCP client implementation.
This module provides the client-side functionality for the IMCP protocol,
including connection management, message sending and receiving.
"""

import asyncio
import websockets
import uuid
from loguru import logger
from typing import Optional, Dict, Any
from .protocol import IMCPProtocol, MessageType, ConnectionState

class IMCPClient:
    """IMCP client implementation for protocol communication.
    
    This class handles client-side operations including:
    - Connection establishment and management
    - Message sending and receiving
    - Session management
    - Error handling
    """
    
    def __init__(self, server_url: str, private_key: Optional[bytes] = None):
        """Initialize the IMCP client.
        
        Args:
            server_url: The WebSocket URL of the server to connect to.
            private_key: Optional private key for message signing.
        """
        self.server_url = server_url
        self.protocol = IMCPProtocol(private_key)
        self.websocket: Optional[websockets.WebSocketClientProtocol] = None
        self.session_id: Optional[str] = None
        self.sequence = 0
        logger.info(f"IMCP client initialized for server: {server_url}")

    async def connect(self) -> None:
        """Establish a connection to the IMCP server.
        
        This method:
        1. Establishes a WebSocket connection
        2. Performs the handshake process
        3. Verifies the server's response
        4. Sets up the session
        
        Raises:
            ConnectionError: If connection or handshake fails.
        """
        try:
            # Establish WebSocket connection
            self.websocket = await websockets.connect(self.server_url)
            logger.info("WebSocket connection established")

            # Send handshake message
            handshake = self.protocol.create_handshake_message()
            self.protocol.sign_message(handshake)
            await self.websocket.send(handshake)
            logger.debug("Handshake message sent")

            # Receive and verify server handshake
            response = await self.websocket.recv()
            server_handshake = self.protocol.verify_message(response)
            if not server_handshake:
                raise ConnectionError("Invalid server handshake")
            
            self.session_id = server_handshake["session_id"]
            logger.info("Connection established successfully")
            
        except Exception as e:
            logger.error(f"Connection failed: {str(e)}")
            raise ConnectionError(f"Failed to connect: {str(e)}")

    async def send(self, data: bytes) -> None:
        """Send a data message to the server.
        
        Args:
            data: The data payload to send.
            
        Raises:
            ConnectionError: If not connected or sending fails.
        """
        if not self.websocket:
            raise ConnectionError("Not connected to server")
            
        try:
            message = self.protocol.create_data_message(data, self.session_id, self.sequence)
            self.protocol.sign_message(message)
            await self.websocket.send(message)
            self.sequence += 1
            logger.debug(f"Data message sent (sequence: {self.sequence})")
        except Exception as e:
            logger.error(f"Failed to send message: {str(e)}")
            raise ConnectionError(f"Failed to send message: {str(e)}")

    async def receive(self) -> Dict[str, Any]:
        """Receive a message from the server.
        
        Returns:
            Dict[str, Any]: The received message.
            
        Raises:
            ConnectionError: If not connected or receiving fails.
        """
        if not self.websocket:
            raise ConnectionError("Not connected to server")
            
        try:
            message = await self.websocket.recv()
            if not self.protocol.verify_message(message):
                raise ConnectionError("Invalid message signature")
            logger.debug("Message received and verified")
            return message
        except Exception as e:
            logger.error(f"Failed to receive message: {str(e)}")
            raise ConnectionError(f"Failed to receive message: {str(e)}")

    async def send_control(self, control_type: str, parameters: Dict[str, str]) -> None:
        """Send a control message to the server.
        
        Args:
            control_type: The type of control command.
            parameters: Additional parameters for the control command.
            
        Raises:
            ConnectionError: If not connected or sending fails.
        """
        if not self.websocket:
            raise ConnectionError("Not connected to server")
            
        try:
            message = self.protocol.create_control_message(control_type, parameters)
            self.protocol.sign_message(message)
            await self.websocket.send(message)
            logger.debug(f"Control message sent (type: {control_type})")
        except Exception as e:
            logger.error(f"Failed to send control message: {str(e)}")
            raise ConnectionError(f"Failed to send control message: {str(e)}")

    async def close(self) -> None:
        """Close the connection to the server."""
        if self.websocket:
            await self.websocket.close()
            self.websocket = None
            self.session_id = None
            self.sequence = 0
            logger.info("Connection closed")

    async def __aenter__(self):
        """Context manager entry point."""
        await self.connect()
        return self

    async def __aexit__(self, exc_type, exc_val, exc_tb):
        """Context manager exit point."""
        await self.close() 