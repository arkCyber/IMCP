# IMCP Protocol Analyzer

A Terminal User Interface (TUI) based protocol analyzer for monitoring and analyzing IMCP protocol messages in real-time.

## Features

- Real-time monitoring of IMCP protocol messages
- Interactive TUI interface with message list and details view
- Support for both incoming and outgoing messages
- Message filtering capabilities
- Detailed message inspection with JSON formatting
- Bilingual interface (English/Chinese)
- WebSocket-based message capture

## Message Types

The analyzer supports the following IMCP message types:
- Handshake
- Data
- Control
- Heartbeat
- Error

## Installation

```bash
# Clone the repository
git clone <repository-url>

# Navigate to the analyzer directory
cd examples/imcp-analyzer

# Build the project
cargo build --release
```

## Usage

1. Start the analyzer:
```bash
cargo run --release
```

2. The analyzer will start listening on port 8765 by default.

3. Help Menu (Press 'h' to toggle):
```
Navigation:
  ↑/↓: Select message
  Enter: View details

Commands:
  q: Quit
  h: Toggle help
  c: Clear messages
  f: Filter messages (TODO)

Status:
  Port: 8765
  Messages: 0
```

## Interface Layout

The analyzer interface is divided into four main sections:
1. Title bar
2. Message list (showing timestamp, direction, type, and session ID)
3. Message details (showing full message content in JSON format)
4. Help information (toggleable)

## Message Display

Each message in the list shows:
- Direction indicator (← for incoming, → for outgoing)
- Timestamp
- Message type
- Session ID

## Dependencies

- [tui-rs](https://github.com/fdehau/tui-rs) - Terminal UI library
- [crossterm](https://github.com/crossterm-rs/crossterm) - Terminal handling
- [tungstenite](https://github.com/snapview/tungstenite-rs) - WebSocket implementation
- [serde_json](https://github.com/serde-rs/json) - JSON serialization/deserialization
- [chrono](https://github.com/chronotope/chrono) - Date and time handling

## License

[Specify your license here]

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. 