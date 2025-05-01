/**
 * IMCP Protocol Example
 * This file demonstrates how to use the IMCP client and server
 */
import { IMCPClient } from './client';
import { IMCPServer } from './server';
import { IMCPMessage } from './types';

// Create and start the server
const server = new IMCPServer(8080);
console.log('IMCP server started on port 8080');

// Create a client instance
const client = new IMCPClient({
    host: 'localhost',
    port: 8080,
    secure: false
});

// Connect to the server
client.connect().then(() => {
    console.log('Client connected to server');

    // Create and send a text message
    const message: IMCPMessage = {
        type: 'text',
        content: 'Hello, IMCP!',
        timestamp: Date.now(),
        sender: 'client1',
        receiver: 'all'
    };

    // Send the message and handle the result
    client.sendMessage(message).then(() => {
        console.log('Message sent successfully');
    }).catch((error) => {
        console.error('Failed to send message:', error);
    });

    // Register handlers for different message types
    client.onMessage('text', (message) => {
        console.log('Received text message:', message);
    });

    client.onMessage('file', (message) => {
        console.log('Received file message:', message);
    });

    client.onMessage('image', (message) => {
        console.log('Received image message:', message);
    });

}).catch((error) => {
    console.error('Failed to connect:', error);
});

// Handle graceful shutdown
process.on('SIGINT', () => {
    console.log('Shutting down...');
    client.disconnect();
    server.close();
    process.exit(0);
}); 