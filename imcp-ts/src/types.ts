/**
 * IMCP (Instant Message Communication Protocol) TypeScript Implementation
 * This file contains all the type definitions used in the IMCP protocol
 */

/**
 * Supported message types in the IMCP protocol
 */
export type MessageType = 'text' | 'file' | 'image' | 'video' | 'audio';

/**
 * Base message interface that all message types must implement
 */
export interface Message {
    /** Type of the message */
    type: MessageType;
    /** Content of the message */
    content: string;
    /** Timestamp when the message was created */
    timestamp: number;
    /** ID of the sender */
    sender: string;
    /** ID of the receiver (use 'all' for broadcast messages) */
    receiver: string;
    /** Optional metadata for additional information */
    metadata?: Record<string, any>;
}

/**
 * Interface for file transfer messages
 */
export interface FileMessage extends Message {
    /** Must be 'file' for file messages */
    type: 'file';
    /** Name of the file */
    fileName: string;
    /** Size of the file in bytes */
    fileSize: number;
    /** MIME type of the file */
    fileType: string;
}

/**
 * Interface for media messages (image, video, audio)
 */
export interface MediaMessage extends Message {
    /** Type of media (image, video, or audio) */
    type: 'image' | 'video' | 'audio';
    /** URL where the media can be accessed */
    mediaUrl: string;
    /** Optional URL for a thumbnail/preview of the media */
    thumbnailUrl?: string;
    /** Optional duration of the media in seconds */
    duration?: number;
}

/**
 * Union type representing all possible message types
 */
export type IMCPMessage = Message | FileMessage | MediaMessage;

/**
 * Configuration options for connecting to an IMCP server
 */
export interface ConnectionConfig {
    /** Hostname or IP address of the server */
    host: string;
    /** Port number of the server */
    port: number;
    /** Whether to use secure WebSocket (wss://) */
    secure?: boolean;
    /** Whether to automatically reconnect on connection loss */
    reconnect?: boolean;
    /** Interval in milliseconds between reconnection attempts */
    reconnectInterval?: number;
}

/**
 * Custom error type for IMCP-specific errors
 */
export interface IMCPError extends Error {
    /** Error code for identifying the type of error */
    code: string;
    /** Optional additional error details */
    details?: any;
} 