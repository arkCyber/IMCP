"""
IMCP server implementation.
This module provides the server-side functionality for the IMCP protocol,
including connection handling, message processing, and client management.
"""

import asyncio
import websockets
import uuid
from typing import Optional, Dict, Any, Set, Callable
from loguru import logger
from .protocol import IMCPProtocol, MessageType, ConnectionState, ErrorCode

class IMCPServer:
    """IMCP server implementation for protocol communication.
    
    This class handles server-side operations including:
    - Client connection management
    - Message processing and routing
    - Session management
    - Error handling
    """
    
    def __init__(self, host: str = "0.0.0.0", port: int = 8765, private_key: Optional[bytes] = None):
        """Initialize IMCP server"""
        self.host = host
        self.port = port
        self.protocol = IMCPProtocol(private_key)
        self.clients: Set[websockets.WebSocketServerProtocol] = set()
        self.client_sessions = {}  # Map of client public keys to session info
        self.message_handlers: Dict[MessageType, Callable] = {}
        logger.info(f"IMCP Server initialized on {host}:{port}")

    def register_handler(self, message_type: MessageType, handler: Callable) -> None:
        """Register a handler for a specific message type.
        
        Args:
            message_type: The type of message to handle.
            handler: The function to call when a message of this type is received.
        """
        self.message_handlers[message_type] = handler
        logger.debug(f"Registered handler for message type: {message_type}")

    async def handle_client(self, websocket: websockets.WebSocketServerProtocol):
        """Handle a new client connection"""
        try:
            # Receive client handshake
            handshake_data = await websocket.recv()
            client_handshake, valid = self.protocol.unpack_message(handshake_data)
            
            if not valid:
                raise ValueError("Invalid client handshake signature")

            # Verify handshake
            if client_handshake["header"]["type"] != MessageType.HANDSHAKE:
                raise ValueError("Invalid handshake message")

            # Store client info
            client_public_key = client_handshake["body"]["public_key"]
            session_id = uuid.uuid4().bytes
            self.client_sessions[client_public_key] = {
                "websocket": websocket,
                "session_id": session_id,
                "sequence": 0
            }

            # Send server handshake
            handshake = self.protocol.create_handshake_message()
            await websocket.send(self.protocol.pack_message(handshake))

            # Establish secure connection
            session_key = await self.protocol.establish_connection(client_public_key)
            self.protocol.state = ConnectionState.ESTABLISHED
            self.clients.add(websocket)
            logger.info(f"New client connected: {client_public_key.hex()}")

            try:
                while True:
                    # Receive and process messages
                    data = await websocket.recv()
                    message, valid = self.protocol.unpack_message(data, session_key)
                    
                    if not valid:
                        raise ValueError("Invalid message signature")

                    # Process message based on type
                    if message["header"]["type"] == MessageType.DATA:
                        await self.handle_data_message(message, client_public_key)
                    elif message["header"]["type"] == MessageType.CONTROL:
                        await self.handle_control_message(message, client_public_key)
                    elif message["header"]["type"] == MessageType.HEARTBEAT:
                        await self.handle_heartbeat(message, client_public_key)

            except websockets.exceptions.ConnectionClosed:
                logger.info(f"Client disconnected: {client_public_key.hex()}")
            finally:
                self.clients.remove(websocket)
                del self.client_sessions[client_public_key]

        except Exception as e:
            logger.error(f"Error handling client: {e}")

    async def handle_data_message(self, message: Dict[str, Any], client_public_key: bytes):
        """Handle incoming data message"""
        session_info = self.client_sessions[client_public_key]
        session_key = self.protocol.get_connection(client_public_key)

        # Process the data (implement your business logic here)
        response = {
            "status": "success",
            "message": "Data received and processed",
            "original": message["body"]["data"]
        }

        # Create and send response
        response_message = self.protocol.create_data_message(
            data=str(response).encode(),
            session_id=session_info["session_id"],
            sequence=session_info["sequence"]
        )
        packed = self.protocol.pack_message(response_message, session_key)
        await session_info["websocket"].send(packed)
        session_info["sequence"] += 1

    async def handle_control_message(self, message: Dict[str, Any], client_public_key: bytes):
        """Handle incoming control message"""
        session_info = self.client_sessions[client_public_key]
        session_key = self.protocol.get_connection(client_public_key)

        # Process control command
        control_type = message["body"]["control_type"]
        parameters = message["body"]["parameters"]

        # Create response
        response = {
            "status": "success",
            "command": control_type,
            "result": f"Command {control_type} processed"
        }

        # Send response
        response_message = self.protocol.create_control_message(
            control_type="RESPONSE",
            target="client",
            parameters=response
        )
        packed = self.protocol.pack_message(response_message, session_key)
        await session_info["websocket"].send(packed)

    async def handle_heartbeat(self, message: Dict[str, Any], client_public_key: bytes):
        """Handle heartbeat message"""
        session_info = self.client_sessions[client_public_key]
        session_key = self.protocol.get_connection(client_public_key)

        # Send heartbeat response
        response = self.protocol.create_control_message(
            control_type="HEARTBEAT",
            target="client",
            parameters={"status": "alive"}
        )
        packed = self.protocol.pack_message(response, session_key)
        await session_info["websocket"].send(packed)

    async def start(self):
        """Start the IMCP server"""
        logger.info(f"Starting IMCP server on {self.host}:{self.port}")
        async with websockets.serve(self.handle_client, self.host, self.port):
            await asyncio.Future()  # run forever

    async def stop(self):
        """Stop the IMCP server"""
        for client in self.clients:
            await client.close()
        self.clients.clear()
        self.client_sessions.clear()
        logger.info("IMCP server stopped") 