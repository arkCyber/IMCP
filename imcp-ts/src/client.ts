/**
 * IMCP Client Implementation
 * This class provides the client-side functionality for the IMCP protocol
 */
import WebSocket from 'ws';
import { IMCPMessage, ConnectionConfig, IMCPError } from './types';

export class IMCPClient {
    /** WebSocket connection instance */
    private ws: WebSocket | null = null;
    /** Client configuration */
    private config: ConnectionConfig;
    /** Map of message type to handler functions */
    private messageHandlers: Map<string, (message: IMCPMessage) => void> = new Map();
    /** Timer for reconnection attempts */
    private reconnectTimer: NodeJS.Timeout | null = null;

    /**
     * Creates a new IMCP client
     * @param config - Configuration options for the client
     */
    constructor(config: ConnectionConfig) {
        this.config = {
            secure: false,
            reconnect: true,
            reconnectInterval: 5000,
            ...config
        };
    }

    /**
     * Establishes a connection to the IMCP server
     * @returns Promise that resolves when the connection is established
     */
    public connect(): Promise<void> {
        return new Promise((resolve, reject) => {
            const protocol = this.config.secure ? 'wss' : 'ws';
            const url = `${protocol}://${this.config.host}:${this.config.port}`;

            this.ws = new WebSocket(url);

            // Handle connection established
            this.ws.on('open', () => {
                console.log('Connected to IMCP server');
                if (this.reconnectTimer) {
                    clearTimeout(this.reconnectTimer);
                    this.reconnectTimer = null;
                }
                resolve();
            });

            // Handle incoming messages
            this.ws.on('message', (data: string) => {
                try {
                    const message: IMCPMessage = JSON.parse(data);
                    this.handleMessage(message);
                } catch (error) {
                    console.error('Error parsing message:', error);
                }
            });

            // Handle connection closed
            this.ws.on('close', () => {
                console.log('Connection closed');
                if (this.config.reconnect) {
                    this.reconnectTimer = setTimeout(() => {
                        console.log('Attempting to reconnect...');
                        this.connect().catch(console.error);
                    }, this.config.reconnectInterval);
                }
            });

            // Handle connection errors
            this.ws.on('error', (error: Error) => {
                console.error('WebSocket error:', error);
                reject(error);
            });
        });
    }

    /**
     * Closes the connection to the server
     */
    public disconnect(): void {
        if (this.ws) {
            this.ws.close();
            this.ws = null;
        }
        if (this.reconnectTimer) {
            clearTimeout(this.reconnectTimer);
            this.reconnectTimer = null;
        }
    }

    /**
     * Sends a message to the server
     * @param message - The message to send
     * @returns Promise that resolves when the message is sent
     */
    public sendMessage(message: IMCPMessage): Promise<void> {
        return new Promise((resolve, reject) => {
            if (!this.ws || this.ws.readyState !== WebSocket.OPEN) {
                reject(new Error('Not connected to server'));
                return;
            }

            try {
                this.ws.send(JSON.stringify(message), (error: Error | undefined) => {
                    if (error) {
                        reject(error);
                    } else {
                        resolve();
                    }
                });
            } catch (error) {
                reject(error);
            }
        });
    }

    /**
     * Registers a handler for a specific message type
     * @param type - The message type to handle
     * @param handler - The function to call when a message of the specified type is received
     */
    public onMessage(type: string, handler: (message: IMCPMessage) => void): void {
        this.messageHandlers.set(type, handler);
    }

    /**
     * Internal method to handle incoming messages
     * @param message - The received message
     */
    private handleMessage(message: IMCPMessage): void {
        const handler = this.messageHandlers.get(message.type);
        if (handler) {
            handler(message);
        }
    }
} 