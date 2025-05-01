/**
 * IMCP Chat Client Example
 * 
 * This file demonstrates how to create a simple chat client using the IMCP protocol.
 * The client connects to the server, sends messages, and displays received messages.
 */

import { IMCPClient, IMCPMessage } from 'imcp-ts';
import * as readline from 'readline';

// Create a readline interface for handling user input
const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

// Create a new IMCP client instance
const client = new IMCPClient({
    host: 'localhost',
    port: 8080
});

// Generate a random user ID for this client
const userId = `user_${Math.floor(Math.random() * 1000)}`;

// Connect to the server and set up message handling
client.connect().then(() => {
    console.log(`Connected to chat server as ${userId}`);

    // Register a handler for text messages
    client.onMessage('text', (message: IMCPMessage) => {
        // Don't display messages sent by this client
        if (message.sender !== userId) {
            console.log(`\n${message.sender}: ${message.content}`);
            rl.prompt();
        }
    });

    // Set up the command line prompt
    rl.setPrompt('> ');
    rl.prompt();

    // Handle user input
    rl.on('line', (line: string) => {
        if (line.trim()) {
            // Create and send a new message
            const message: IMCPMessage = {
                type: 'text',
                content: line.trim(),
                timestamp: Date.now(),
                sender: userId,
                receiver: 'all'
            };

            client.sendMessage(message).catch((error: Error) => {
                console.error('Failed to send message:', error);
            });
        }
        rl.prompt();
    });

}).catch((error: Error) => {
    console.error('Failed to connect:', error);
    process.exit(1);
});

// Handle graceful shutdown on SIGINT (Ctrl+C)
process.on('SIGINT', () => {
    console.log('\nDisconnecting from chat server...');
    client.disconnect();
    rl.close();
    process.exit(0);
}); 