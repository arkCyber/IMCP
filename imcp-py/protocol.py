"""
IMCP (Industrial Model Context Protocol) core implementation.
This module provides the core protocol functionality including message handling,
security features, and connection management.
"""

import nacl.secret
import nacl.signing
import nacl.encoding
import nacl.utils
import msgpack
import time
from typing import Optional, Dict, Any, Tuple
from enum import IntEnum
from loguru import logger
import uuid

class MessageType(IntEnum):
    """Types of messages supported by the protocol."""
    HANDSHAKE = 0x01
    DATA = 0x02
    CONTROL = 0x03
    HEARTBEAT = 0x04
    ERROR = 0xFF

class ConnectionState(IntEnum):
    """Current state of the connection."""
    INIT = 0
    HANDSHAKE = 1
    AUTHENTICATED = 2
    ESTABLISHED = 3
    CLOSING = 4
    CLOSED = 5

class ErrorCode(IntEnum):
    """Error codes for protocol operations."""
    SUCCESS = 0x0000
    INVALID_MESSAGE = 0x0001
    AUTH_FAILED = 0x0002
    ENCRYPTION_ERROR = 0x0003
    SESSION_ERROR = 0x0004
    PROTOCOL_ERROR = 0x0005
    INTERNAL_ERROR = 0xFFFF

class IMCPProtocol:
    """Core protocol implementation for IMCP.
    
    This class handles the core functionality of the IMCP protocol including:
    - Message creation and handling
    - Security operations (encryption, signing)
    - Connection state management
    - Session key management
    """
    
    def __init__(self, private_key: Optional[bytes] = None):
        """Initialize the IMCP protocol instance.
        
        Args:
            private_key: Optional private key for message signing. If not provided,
                        a new key pair will be generated.
        """
        if private_key:
            self.signing_key = nacl.signing.SigningKey(private_key)
        else:
            self.signing_key = nacl.signing.SigningKey.generate()
        
        self.verify_key = self.signing_key.verify_key
        self.public_key = self.verify_key.encode()
        self.connection_pool = {}
        self.state = ConnectionState.INIT
        logger.info("IMCP Protocol initialized")

    def generate_session_key(self) -> bytes:
        """Generate a new session key for symmetric encryption"""
        return nacl.utils.random(nacl.secret.SecretBox.KEY_SIZE)

    def encrypt_message(self, message: Dict[str, Any], session_key: bytes) -> bytes:
        """Encrypt a message using the session key"""
        box = nacl.secret.SecretBox(session_key)
        serialized = msgpack.packb(message)
        return box.encrypt(serialized)

    def decrypt_message(self, encrypted_data: bytes, session_key: bytes) -> Dict[str, Any]:
        """Decrypt a message using the session key"""
        box = nacl.secret.SecretBox(session_key)
        decrypted = box.decrypt(encrypted_data)
        return msgpack.unpackb(decrypted)

    def sign_message(self, message: Dict[str, Any]) -> bytes:
        """Sign a message using the private key"""
        serialized = msgpack.packb(message)
        return self.signing_key.sign(serialized).signature

    def verify_message(self, message: Dict[str, Any], signature: bytes) -> bool:
        """Verify a message signature"""
        try:
            serialized = msgpack.packb(message)
            self.verify_key.verify(serialized, signature)
            return True
        except nacl.exceptions.BadSignatureError:
            return False

    def create_handshake_message(self) -> Dict[str, Any]:
        """Create a handshake message"""
        return {
            "header": {
                "version": "1.0",
                "type": MessageType.HANDSHAKE,
                "timestamp": int(time.time() * 1000),
                "sequence": 0
            },
            "body": {
                "public_key": self.public_key,
                "capabilities": ["encryption", "compression", "authentication"]
            }
        }

    def create_data_message(self, data: bytes, session_id: bytes, sequence: int) -> Dict[str, Any]:
        """Create a data message"""
        return {
            "header": {
                "version": "1.0",
                "type": MessageType.DATA,
                "timestamp": int(time.time() * 1000),
                "sequence": sequence
            },
            "body": {
                "session_id": session_id,
                "data_type": "binary",
                "format": "raw",
                "data": data
            }
        }

    def create_control_message(self, control_type: str, target: str, parameters: Dict[str, Any]) -> Dict[str, Any]:
        """Create a control message"""
        return {
            "header": {
                "version": "1.0",
                "type": MessageType.CONTROL,
                "timestamp": int(time.time() * 1000),
                "sequence": 0
            },
            "body": {
                "control_type": control_type,
                "target": target,
                "parameters": parameters
            }
        }

    def create_error_message(self, code: ErrorCode, message: str) -> Dict[str, Any]:
        """Create an error message"""
        return {
            "header": {
                "version": "1.0",
                "type": MessageType.ERROR,
                "timestamp": int(time.time() * 1000),
                "sequence": 0
            },
            "body": {
                "code": code,
                "message": message
            }
        }

    def pack_message(self, message: Dict[str, Any], session_key: Optional[bytes] = None) -> bytes:
        """Pack a message for transmission"""
        if session_key:
            encrypted = self.encrypt_message(message, session_key)
            signature = self.sign_message(message)
            return signature + encrypted
        else:
            return msgpack.packb(message)

    def unpack_message(self, data: bytes, session_key: Optional[bytes] = None) -> Tuple[Dict[str, Any], bool]:
        """Unpack a received message"""
        if session_key:
            signature = data[:64]
            encrypted = data[64:]
            message = self.decrypt_message(encrypted, session_key)
            is_valid = self.verify_message(message, signature)
            return message, is_valid
        else:
            return msgpack.unpackb(data), True

    async def establish_connection(self, peer_public_key: bytes) -> bytes:
        """Establish a secure connection with a peer"""
        session_key = self.generate_session_key()
        self.connection_pool[peer_public_key] = session_key
        self.state = ConnectionState.ESTABLISHED
        return session_key

    def get_connection(self, peer_public_key: bytes) -> Optional[bytes]:
        """Get an existing connection's session key"""
        return self.connection_pool.get(peer_public_key)

    async def send_message(self, peer_public_key: bytes, message: Dict[str, Any]) -> bytes:
        """Send a secure message to a peer"""
        session_key = self.get_connection(peer_public_key)
        if not session_key:
            session_key = await self.establish_connection(peer_public_key)
        
        signature = self.sign_message(message)
        encrypted = self.encrypt_message(message, session_key)
        return signature + encrypted

    async def receive_message(self, peer_public_key: bytes, data: bytes) -> Dict[str, Any]:
        """Receive and process a secure message from a peer"""
        session_key = self.get_connection(peer_public_key)
        if not session_key:
            raise ValueError("No active connection with peer")

        signature = data[:64]  # Ed25519 signature is 64 bytes
        encrypted = data[64:]
        
        message = self.decrypt_message(encrypted, session_key)
        if not self.verify_message(message, signature):
            raise ValueError("Invalid message signature")
        
        return message 