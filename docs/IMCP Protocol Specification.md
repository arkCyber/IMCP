#### Protocol Version
- Current Version: 1.0
- Protocol Identifier: IMCP/1.0

#### Protocol Stack
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

#### Protocol Features

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

#### Protocol Messages

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

#### Protocol States

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

#### Error Codes

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

#### Protocol Flow

1. **Connection Establishment**
   ```
   Client                     Server
   ------                     ------
   2. INIT
   3. Send HANDSHAKE -------> 3. Receive HANDSHAKE
   4. Verify Server          5. Verify Client
   5. Generate Session Key   7. Generate Session Key
   6. ESTABLISHED <--------> 9. ESTABLISHED
   ```

7. **Data Transfer**
   ```
   Sender                     Receiver
   ------                     --------
   8. Create Message
   9. Sign Message
   10. Encrypt Message
   11. Send Message --------> 5. Receive Message
                            6. Verify Signature
                            7. Decrypt Message
                            8. Process Message
   ```

12. **Connection Termination**
   ```
   Initiator                 Responder
   ---------                 ---------
   13. Send CLOSE --------> 2. Receive CLOSE
   14. Wait for ACK        4. Send ACK
   15. CLOSED <----------- 6. CLOSED
   ```

#### Implementation Requirements

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
