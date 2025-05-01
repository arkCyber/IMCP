/**
 * IMCP Server Implementation
 * This class provides the server-side functionality for the IMCP protocol
 */
import WebSocket, { WebSocketServer } from 'ws';
import { IMCPMessage } from './types';

export class IMCPServer {
    /** WebSocket server instance */
    private wss: WebSocketServer;
    /** Map of client IDs to WebSocket connections */
    private clients: Map<string, WebSocket> = new Map();

    /**
     * Creates a new IMCP server
     * @param port - The port number to listen on
     */
    constructor(port: number) {
        this.wss = new WebSocketServer({ port });

        // Handle new client connections
        this.wss.on('connection', (ws: WebSocket) => {
            console.log('New client connected');

            // Handle incoming messages from client
            ws.on('message', (data: string) => {
                try {
                    const message: IMCPMessage = JSON.parse(data);
                    this.handleMessage(ws, message);
                } catch (error) {
                    console.error('Error parsing message:', error);
                }
            });

            // Handle client disconnection
            ws.on('close', () => {
                this.removeClient(ws);
                console.log('Client disconnected');
            });

            // Handle client errors
            ws.on('error', (error: Error) => {
                console.error('WebSocket error:', error);
                this.removeClient(ws);
            });
        });
    }

    /**
     * Handles incoming messages based on their type
     * @param ws - The WebSocket connection that sent the message
     * @param message - The received message
     */
    private handleMessage(ws: WebSocket, message: IMCPMessage): void {
        // Handle different message types
        switch (message.type) {
            case 'text':
                this.broadcastMessage(message);
                break;
            case 'file':
                this.handleFileMessage(message);
                break;
            case 'image':
            case 'video':
            case 'audio':
                this.handleMediaMessage(message);
                break;
            default:
                console.warn('Unknown message type:', message.type);
        }
    }

    /**
     * Broadcasts a message to all connected clients
     * @param message - The message to broadcast
     */
    private broadcastMessage(message: IMCPMessage): void {
        this.wss.clients.forEach((client: WebSocket) => {
            if (client.readyState === WebSocket.OPEN) {
                client.send(JSON.stringify(message));
            }
        });
    }

    /**
     * Handles file transfer messages
     * @param message - The file message to handle
     */
    private handleFileMessage(message: IMCPMessage): void {
        // Implement file handling logic
        this.broadcastMessage(message);
    }

    /**
     * Handles media messages (image, video, audio)
     * @param message - The media message to handle
     */
    private handleMediaMessage(message: IMCPMessage): void {
        // Implement media handling logic
        this.broadcastMessage(message);
    }

    /**
     * Removes a client from the clients map
     * @param ws - The WebSocket connection to remove
     */
    private removeClient(ws: WebSocket): void {
        for (const [id, client] of this.clients.entries()) {
            if (client === ws) {
                this.clients.delete(id);
                break;
            }
        }
    }

    /**
     * Closes the server and all client connections
     */
    public close(): void {
        this.wss.close();
    }
} 