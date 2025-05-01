# IMCP TypeScript Implementation

This is a TypeScript implementation of the IMCP (Instant Message Communication Protocol) protocol.

## Installation

```bash
npm install imcp-ts
```

## Usage

### Server

```typescript
import { IMCPServer } from 'imcp-ts';

const server = new IMCPServer(8080);
console.log('IMCP server started on port 8080');
```

### Client

```typescript
import { IMCPClient } from 'imcp-ts';
import { IMCPMessage } from 'imcp-ts';

const client = new IMCPClient({
  host: 'localhost',
  port: 8080,
  secure: false
});

// Connect to server
client.connect().then(() => {
  console.log('Connected to server');

  // Send a message
  const message: IMCPMessage = {
    type: 'text',
    content: 'Hello, IMCP!',
    timestamp: Date.now(),
    sender: 'client1',
    receiver: 'all'
  };

  client.sendMessage(message);
});

// Handle incoming messages
client.onMessage('text', (message) => {
  console.log('Received text message:', message);
});
```

## Features

- WebSocket-based communication
- Support for different message types (text, file, image, video, audio)
- Automatic reconnection
- Type-safe message handling
- Event-based message processing

## Message Types

The protocol supports the following message types:

- `text`: Simple text messages
- `file`: File transfer messages
- `image`: Image messages
- `video`: Video messages
- `audio`: Audio messages

## Development

To build the project:

```bash
npm run build
```

To run tests:

```bash
npm test
```

To run the linter:

```bash
npm run lint
```

## License

MIT 