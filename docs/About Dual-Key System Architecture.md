### 
The Dual-Key System is a fundamental security feature of IMCP, where both the client and server maintain their own public/private key pairs. This architecture provides enhanced security and authentication capabilities.

#### Key Components

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

#### Security Benefits

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

#### Implementation Details

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

#### Use Cases

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