# IMCP Protocol Rust Examples

This directory contains Rust example implementations demonstrating the usage of the IMCP (Industrial Model Context Protocol).

## Prerequisites

Before running any examples, ensure you have the required environment:

1. Install Rust and Cargo:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Verify the installation:
   ```bash
   rustc --version
   cargo --version
   ```

## Project Structure

```
examples/rust/
├── Cargo.toml        # Project configuration and dependencies
├── Cargo.lock        # Dependency lock file
├── src/              # Source code directory
├── secure_communication.rs    # Secure communication example
└── industrial_control.rs      # Industrial control example
```

## Building and Running Examples

1. Navigate to the examples directory:
   ```bash
   cd examples/rust
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run specific examples:
   ```bash
   # Run secure communication example
   cargo run --example secure_communication

   # Run industrial control example
   cargo run --example industrial_control
   ```

## Example Descriptions

### 1. Secure Communication Example (`secure_communication.rs`)
This example demonstrates:
- Secure client-server communication setup
- Message encryption and decryption
- Secure control command handling
- Error handling and logging

To run:
```bash
cargo run --example secure_communication
```

Expected output:
```
[INFO] Server started on localhost:8766
[INFO] Client connected successfully
[INFO] Received encrypted response
[INFO] Received secure response
```

### 2. Industrial Control Example (`industrial_control.rs`)
This example demonstrates:
- Industrial device simulation
- Real-time monitoring
- Control command handling
- Device status management

To run:
```bash
cargo run --example industrial_control
```

## Dependencies

The examples use the following Rust crates:
- `imcp`: IMCP protocol implementation
- `tokio`: Async runtime
- `log`: Logging framework
- `env_logger`: Logging configuration
- `async-trait`: Async trait support

## Development Tips

1. Enable debug logging:
   ```bash
   RUST_LOG=debug cargo run --example secure_communication
   ```

2. Run with release optimizations:
   ```bash
   cargo run --release --example secure_communication
   ```

3. Check for warnings:
   ```bash
   cargo check
   ```

## Troubleshooting

1. If you encounter build errors:
   - Ensure all dependencies are up to date: `cargo update`
   - Clean the build: `cargo clean`
   - Rebuild: `cargo build`

2. If you encounter runtime errors:
   - Check if the required ports are available
   - Verify network connectivity
   - Check log output for detailed error messages

## Notes

1. All examples use localhost for demonstration purposes. For production use, replace with appropriate host addresses.
2. The examples use different ports (8765, 8766, 8767) to avoid conflicts when running multiple examples.
3. The industrial control example simulates device behavior. In a real implementation, this would be replaced with actual device communication.
4. Security keys in the examples are generated for demonstration. In production, proper key management should be implemented. 