/**
 * IMCP Chat Server Example
 * 
 * This file demonstrates how to create a simple chat server using the IMCP protocol.
 * The server listens for incoming connections and broadcasts messages to all connected clients.
 */

import { IMCPServer } from 'imcp-ts';

// Create a new IMCP server instance listening on port 8080
const server = new IMCPServer(8080);
console.log('Chat server is running on port 8080');

// Handle graceful shutdown on SIGINT (Ctrl+C)
process.on('SIGINT', () => {
    console.log('\nShutting down server...');
    server.close();
    process.exit(0);
}); 