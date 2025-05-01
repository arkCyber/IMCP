# IMCP (Industrial Model Context Protocol) Implementation

IMCP is an enhanced version of the Model Context Protocol (MCP) designed specifically for industrial applications, focusing on security, performance, and reliability. It serves as a secure and efficient communication protocol for industrial scenarios.

## Protocol Overview

IMCP (Industrial Model Context Protocol) is based on the MCP (Model Context Protocol) architecture, optimized for industrial production scenarios with strict requirements for real-time performance, reliability, and security.

### Key Improvements over MCP

1. **Enhanced Security**
   - Dual-key authentication (both client and server have public/private key pairs)
   - NaCl-based public/private key generation for all nodes
   - Digital signatures for all exchanged data
   - Secure key exchange using Curve25519
   - Message authentication using Poly1305

2. **Performance Optimization**
   - Binary protocol replacing JSON for better efficiency
   - Connection pooling for resource optimization
   - Asynchronous I/O for high throughput
   - Message compression support

3. **Industrial Features**
   - Millisecond-level deterministic response
   - Full lifecycle security compliance
   - Support for industrial protocol stacks

## Features

- **Dual-Key Authentication**: Both client and server maintain their own public/private key pairs
- **Secure Communication**: Uses NaCl cryptography for end-to-end encryption
- **Binary Protocol**: Efficient binary message format using MessagePack
- **Asynchronous I/O**: Built on asyncio for high-performance networking
- **Connection Pooling**: Efficient connection management
- **Digital Signatures**: Message authentication using Ed25519
- **WebSocket Support**: Real-time bidirectional communication
- **Industrial-Grade Security**: Compliance with industrial security standards
- **High Performance**: Optimized for industrial real-time requirements

## Security Architecture

IMCP implements a comprehensive security framework using NaCl cryptography:

### Key Management
- **Node Authentication**: Each node (client/server) has its own public/private key pair
- **Public Key as Node Identifier**: Node's public key serves as its unique identifier
- **Key Exchange**: Curve25519 (Elliptic Curve Diffie-Hellman) for secure session key establishment
- **Symmetric Encryption**: XSalsa20 stream cipher for message encryption
- **Message Authentication**: Poly1305 MAC for message integrity
- **Digital Signatures**: Ed25519 (based on EdDSA) for message signing

### Security Features
- **Mutual Authentication**: Both parties verify each other's identity
- **End-to-End Encryption**: All messages are encrypted using session keys
- **Message Signing**: All messages are digitally signed by the sender
- **Session Key Management**: Secure session key generation and exchange

These algorithms are proven to resist:
- Side-channel attacks
- Timing attacks
- Other common security threats

## Operating Modes

IMCP supports two operating modes:

1. **STDIO Mode (Local Operation)**
   - Used when client and server are on the same host
   - Optimized for local communication
   - Lower latency and higher throughput
   - Still maintains full security with key pairs

2. **SSE Mode (Remote Service)**
   - Used for remote client-server communication
   - Implements additional security measures
   - Optimized for network transmission
   - Full dual-key authentication

## Installation

1. Clone the repository
2. Install dependencies:
```bash
pip install -r requirements.txt
```

## Project Structure

```
imcp/
├── protocol.py    # Core protocol implementation
├── client.py      # Client implementation
├── server.py      # Server implementation
└── examples/      # Example usage
    └── basic_usage.py
```

## Usage

### Starting the Server

```python
from imcp.server import IMCPServer

async def main():
    # Server generates its own key pair if not provided
    server = IMCPServer(host="0.0.0.0", port=8765)
    await server.start()

asyncio.run(main())
```

### Using the Client

```python
from imcp.client import IMCPClient

async def main():
    # Client generates its own key pair if not provided
    async with IMCPClient("ws://localhost:8765") as client:
        message = {
            "type": "request",
            "data": "Hello, IMCP Server!"
        }
        await client.send(message)
        response = await client.receive()
        print(response)

asyncio.run(main())
```

## Security Features

- **Dual-Key System**: Both client and server maintain their own key pairs
- **Key Exchange**: Curve25519 for secure key exchange
- **Encryption**: XSalsa20 for symmetric encryption
- **Authentication**: Ed25519 for digital signatures
- **Message Integrity**: Poly1305 for message authentication
- **Secure Session Management**: Connection pooling with session keys
- **End-to-End Encryption**: All messages are encrypted in transit
- **Mutual Authentication**: Both parties verify each other's identity

## Performance Optimizations

- Binary protocol using MessagePack
- Connection pooling
- Asynchronous I/O with asyncio
- Efficient message serialization
- Message compression support
- Optimized network transmission

## Industrial Applications

IMCP is designed for various industrial scenarios:

- **Manufacturing**: Real-time control and monitoring
- **Energy**: Power grid management and monitoring
- **Transportation**: Traffic control and management
- **Healthcare**: Medical device communication
- **Smart Cities**: Infrastructure management

## Compliance and Standards

IMCP is designed to comply with:
- GDPR
- HIPAA
- IEC 62443
- Other relevant industrial security standards

## License

MIT License

## References

- [NaCl Cryptography Library](https://nacl.cr.yp.to/)
- [Model Context Protocol](https://modelcontextprotocol.io/)
- [WebSocket Protocol](https://websockets.readthedocs.io/)
- [MessagePack](https://msgpack.org/)
- [Industrial Security Standards](https://www.iec.ch/)

## About Dual-Key System Architecture

The Dual-Key System is a fundamental security feature of IMCP, where both the client and server maintain their own public/private key pairs. This architecture provides enhanced security and authentication capabilities.

### Key Components

1. **Node Identity**
   - Each node (client or server) generates its own unique key pair
   - Public key serves as the node's unique identifier
   - Private key is securely stored and never shared

2. **Key Generation**
   - Keys are generated using NaCl's secure key generation
   - Public key: 32 bytes
   - Private key: 64 bytes
   - Keys are generated using cryptographically secure random number generation

3. **Authentication Process**
   - Initial handshake: Exchange of public keys
   - Mutual verification: Both parties verify each other's identity
   - Session key establishment: Secure key exchange using Curve25519
   - Digital signatures: All messages are signed with sender's private key

### Security Benefits

1. **Mutual Authentication**
   - Both client and server verify each other's identity
   - Prevents man-in-the-middle attacks
   - Ensures communication with intended party

2. **Message Security**
   - End-to-end encryption using session keys
   - Digital signatures for message authenticity
   - Protection against message tampering

3. **Identity Management**
   - Public keys serve as unique node identifiers
   - Enables secure node-to-node communication
   - Supports complex network topologies

### Implementation Details

1. **Key Exchange Protocol**
   ```
   Client                     Server
   ------                     ------
   Generate Key Pair          Generate Key Pair
   Send Public Key ---------> Receive Public Key
   Receive Public Key <------ Send Public Key
   Verify Server Identity     Verify Client Identity
   Generate Session Key       Generate Session Key
   ```

2. **Message Flow**
   ```
   Sender                     Receiver
   ------                     --------
   Sign Message with Private Key
   Encrypt with Session Key
   Send Message ------------> Receive Message
                              Decrypt with Session Key
                              Verify Signature with Public Key
   ```

3. **Session Management**
   - Session keys are generated for each connection
   - Keys are periodically rotated for enhanced security
   - Connection pooling maintains active sessions

### Use Cases

1. **Industrial Control Systems**
   - Secure communication between controllers
   - Authentication of control commands
   - Protection of sensitive operational data

2. **Distributed Systems**
   - Secure node-to-node communication
   - Authentication in mesh networks
   - Secure data exchange between components

3. **Cloud Integration**
   - Secure communication with cloud services
   - Authentication of cloud resources
   - Protection of data in transit

## Data Structures and Packet Format

### Basic Data Types

1. **Key Types**
   ```python
   PublicKey = bytes[32]    # 32-byte public key
   PrivateKey = bytes[64]   # 64-byte private key
   SessionKey = bytes[32]   # 32-byte session key
   ```

2. **Message Types**
   ```python
   MessageType = enum {
       HANDSHAKE = 0x01,    # Initial connection handshake
       DATA = 0x02,         # Data message
       CONTROL = 0x03,      # Control message
       HEARTBEAT = 0x04,    # Connection heartbeat
       ERROR = 0xFF         # Error message
   }
   ```

### Packet Structure

1. **Header Format**
   ```
   +----------------+----------------+----------------+----------------+
   | Version (1B)   | Type (1B)      | Length (2B)    | Reserved (4B)  |
   +----------------+----------------+----------------+----------------+
   | Signature (64B)                                                  |
   +-----------------------------------------------------------------+
   | Encrypted Payload (Variable)                                     |
   +-----------------------------------------------------------------+
   ```

2. **Field Descriptions**
   - **Version**: Protocol version (currently 0x01)
   - **Type**: Message type (from MessageType enum)
   - **Length**: Payload length in bytes
   - **Reserved**: Reserved for future use
   - **Signature**: Ed25519 signature of the payload
   - **Encrypted Payload**: XSalsa20 encrypted message data

### Message Formats

1. **Handshake Message**
   ```python
   HandshakeMessage = {
       "type": MessageType.HANDSHAKE,
       "version": "1.0",
       "public_key": PublicKey,
       "timestamp": int64,  # Unix timestamp
       "capabilities": [str] # Supported features
   }
   ```

2. **Data Message**
   ```python
   DataMessage = {
       "type": MessageType.DATA,
       "session_id": bytes[16],  # Session identifier
       "sequence": uint32,       # Message sequence number
       "timestamp": int64,       # Message timestamp
       "payload": bytes          # Actual message data
   }
   ```

3. **Control Message**
   ```python
   ControlMessage = {
       "type": MessageType.CONTROL,
       "command": str,           # Control command
       "parameters": dict,       # Command parameters
       "timestamp": int64        # Command timestamp
   }
   ```

4. **Heartbeat Message**
   ```python
   HeartbeatMessage = {
       "type": MessageType.HEARTBEAT,
       "timestamp": int64,       # Current timestamp
       "status": uint8          # Connection status
   }
   ```

### Packet Exchange Protocol

1. **Connection Establishment**
   ```
   Client                     Server
   ------                     ------
   1. Generate Key Pair       1. Generate Key Pair
   2. Send Handshake -------> 2. Receive Handshake
      (Public Key)              Verify Client
   3. Receive Handshake <--- 3. Send Handshake
      Verify Server             (Public Key)
   4. Generate Session Key    4. Generate Session Key
   ```

2. **Data Exchange**
   ```
   Sender                     Receiver
   ------                     --------
   1. Create Message
   2. Sign with Private Key
   3. Encrypt with Session Key
   4. Send Packet ----------> 5. Receive Packet
                              6. Verify Signature
                              7. Decrypt with Session Key
                              8. Process Message
   ```

3. **Error Handling**
   ```python
   ErrorMessage = {
       "type": MessageType.ERROR,
       "code": uint16,          # Error code
       "message": str,          # Error description
       "timestamp": int64       # Error timestamp
   }
   ```

### Security Considerations

1. **Message Integrity**
   - All messages are signed with sender's private key
   - Signatures are verified using sender's public key
   - Timestamps prevent replay attacks

2. **Encryption**
   - Session keys are established using Curve25519
   - Messages are encrypted using XSalsa20
   - Keys are rotated periodically

3. **Authentication**
   - Initial handshake verifies both parties
   - Each message is authenticated
   - Session keys are unique per connection 

## IMCP Protocol Specification

### Protocol Version
- Current Version: 1.0
- Protocol Identifier: IMCP/1.0

### Protocol Stack
```
+-------------------+
|    Application    |
+-------------------+
|     IMCP Core     |
+-------------------+
|  Security Layer   |
+-------------------+
|  Transport Layer  |
+-------------------+
|    Network Layer  |
+-------------------+
```

### Protocol Features

1. **Security Layer**
   - NaCl-based cryptography
   - Dual-key authentication system
   - End-to-end encryption
   - Message signing and verification
   - Session key management

2. **Transport Layer**
   - Binary protocol format
   - Connection pooling
   - Heartbeat mechanism
   - Error handling
   - Flow control

3. **Application Layer**
   - Message routing
   - Session management
   - State synchronization
   - Service discovery

### Protocol Messages

1. **System Messages**
   ```python
   SystemMessage = {
       "header": {
           "version": "1.0",
           "type": MessageType,
           "timestamp": int64,
           "sequence": uint32
       },
       "body": {
           "command": str,
           "parameters": dict,
           "data": bytes
       }
   }
   ```

2. **Control Messages**
   ```python
   ControlMessage = {
       "header": {
           "version": "1.0",
           "type": MessageType.CONTROL,
           "timestamp": int64,
           "sequence": uint32
       },
       "body": {
           "control_type": str,  # START, STOP, PAUSE, RESUME
           "target": str,        # Target node identifier
           "parameters": dict    # Control parameters
       }
   }
   ```

3. **Data Messages**
   ```python
   DataMessage = {
       "header": {
           "version": "1.0",
           "type": MessageType.DATA,
           "timestamp": int64,
           "sequence": uint32
       },
       "body": {
           "data_type": str,     # Data type identifier
           "format": str,        # Data format (binary, text, etc.)
           "compression": str,   # Compression algorithm if used
           "data": bytes         # Actual data payload
       }
   }
   ```

### Protocol States

1. **Connection States**
   ```python
   ConnectionState = enum {
       INIT = 0,        # Initial state
       HANDSHAKE = 1,   # Performing handshake
       AUTHENTICATED = 2, # Authentication complete
       ESTABLISHED = 3,  # Connection established
       CLOSING = 4,     # Connection closing
       CLOSED = 5       # Connection closed
   }
   ```

2. **Session States**
   ```python
   SessionState = enum {
       NEW = 0,         # New session
       ACTIVE = 1,      # Session active
       PAUSED = 2,      # Session paused
       RESUMED = 3,     # Session resumed
       TERMINATED = 4   # Session terminated
   }
   ```

### Error Codes

```python
ErrorCode = enum {
    SUCCESS = 0x0000,           # Operation successful
    INVALID_MESSAGE = 0x0001,   # Invalid message format
    AUTH_FAILED = 0x0002,       # Authentication failed
    ENCRYPTION_ERROR = 0x0003,  # Encryption/decryption error
    SESSION_ERROR = 0x0004,     # Session management error
    PROTOCOL_ERROR = 0x0005,    # Protocol violation
    INTERNAL_ERROR = 0xFFFF     # Internal server error
}
```

### Protocol Flow

1. **Connection Establishment**
   ```
   Client                     Server
   ------                     ------
   1. INIT
   2. Send HANDSHAKE -------> 3. Receive HANDSHAKE
   4. Verify Server          5. Verify Client
   6. Generate Session Key   7. Generate Session Key
   8. ESTABLISHED <--------> 9. ESTABLISHED
   ```

2. **Data Transfer**
   ```
   Sender                     Receiver
   ------                     --------
   1. Create Message
   2. Sign Message
   3. Encrypt Message
   4. Send Message --------> 5. Receive Message
                            6. Verify Signature
                            7. Decrypt Message
                            8. Process Message
   ```

3. **Connection Termination**
   ```
   Initiator                 Responder
   ---------                 ---------
   1. Send CLOSE --------> 2. Receive CLOSE
   3. Wait for ACK        4. Send ACK
   5. CLOSED <----------- 6. CLOSED
   ```

### Implementation Requirements

1. **Security Requirements**
   - Must implement NaCl cryptography
   - Must support dual-key authentication
   - Must implement message signing
   - Must support session key rotation

2. **Performance Requirements**
   - Maximum handshake time: 100ms
   - Maximum message processing time: 50ms
   - Minimum heartbeat interval: 30 seconds
   - Maximum connection pool size: 1000

3. **Reliability Requirements**
   - Must implement automatic reconnection
   - Must support message retransmission
   - Must maintain message ordering
   - Must implement proper error handling 

# IMCP (Industrial Model Context Protocol)

IMCP是一个用于工业模型上下文通信的协议实现，提供了安全、可靠的消息交换机制。

## 特性

- 安全的WebSocket通信
- 基于Ed25519的数字签名
- 会话管理和状态跟踪
- 消息序列化和反序列化
- 异步操作支持

## 安装

在`Cargo.toml`中添加依赖：

```toml
[dependencies]
imcp = "0.1.0"
```

## 快速开始

### 创建客户端

```rust
use imcp::{new_client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // 创建客户端
    let mut client = new_client()?;
    
    // 连接到服务器
    client.connect("ws://127.0.0.1:8080").await?;
    
    // 发送数据
    let data = b"Hello, IMCP!".to_vec();
    client.send(data).await?;
    
    // 接收响应
    let response = client.receive().await?;
    println!("Received response: {:?}", response);
    
    // 关闭连接
    client.close().await?;
    
    Ok(())
}
```

### 创建服务器

```rust
use imcp::{new_server, DefaultHandler, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // 创建服务器
    let handler = DefaultHandler;
    let mut server = new_server(handler)?;
    
    // 启动服务器
    server.start("127.0.0.1", 8080).await?;
    
    Ok(())
}
```

### 自定义消息处理器

```rust
use imcp::{IMCPHandler, Message, Result};
use std::collections::HashMap;

struct CustomHandler;

#[async_trait::async_trait]
impl IMCPHandler for CustomHandler {
    async fn handle_message(
        &self,
        message: Message,
        client_public_key: &[u8],
    ) -> Result<Message> {
        // 处理消息并返回响应
        Ok(message)
    }
}
```

## 示例

运行示例：

```bash
cargo run --example basic
```

## 文档

详细的API文档可以通过以下命令生成：

```bash
cargo doc --open
```

## 许可证

MIT 