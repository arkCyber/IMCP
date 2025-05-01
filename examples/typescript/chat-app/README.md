# IMCP Chat Application Example

This is a simple command-line chat application example implemented using the IMCP (Instant Message Communication Protocol) protocol.

## Features

- Command-line based user interface
- Real-time message delivery
- Multi-user chat support
- Randomly generated user IDs
- Graceful shutdown handling

## Installation

```bash
# Install dependencies
npm install

# Build the project
npm run build
```

## Running the Example

You can run the example in the following ways:

1. Start the server in one terminal:
```bash
npm run start:server
```

2. Start the client in another terminal:
```bash
npm run start:client
```

Or start both server and client with a single command:
```bash
npm run dev
```

## Usage Instructions

1. After starting the server and client, each client will be assigned a random user ID
2. Type your message in the command line and press Enter to send
3. All connected clients will receive the message
4. Press Ctrl+C to exit the program

## Code Structure

- `src/server.ts`: Chat server implementation
  - Creates an IMCP server instance
  - Handles client connections
  - Broadcasts messages to all clients
  - Implements graceful shutdown

- `src/client.ts`: Chat client implementation
  - Creates an IMCP client instance
  - Handles user input through command line
  - Displays received messages
  - Manages connection lifecycle
  - Implements graceful shutdown

## Technical Details

- Uses WebSocket for real-time communication
- Implements the IMCP protocol for message exchange
- Provides a simple command-line interface using Node.js readline
- Handles connection errors and reconnection attempts
- Supports graceful shutdown on SIGINT signal

## Notes

- This is a basic example implementation for demonstrating IMCP protocol usage
- In a production environment, you might want to add:
  - User authentication
  - Message persistence
  - Message encryption
  - Error recovery mechanisms
  - Rate limiting
  - User presence indicators
- This example uses the local IMCP TypeScript implementation 