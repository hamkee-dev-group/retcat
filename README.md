# retcat

A simple netcat implementation in Rust, providing TCP and UDP networking capabilities for data transfer and network communication.

## Features

- **TCP Client/Server**: Connect to or listen for TCP connections
- **UDP Client/Server**: Send/receive UDP packets
- **Bidirectional Data Transfer**: Full-duplex communication between stdin/stdout and network
- **Cross-platform**: Works on Linux, macOS, and Windows

## Installation

### From Source

```bash
git clone [retcat](https://github.com/hamkee-dev-group/retract)
cd retcat
cargo build --release
```

The binary will be available at `target/release/retcat`.

## Usage

### TCP Server (Listen Mode)

```bash
# Listen on port 8080
retcat -l -p 8080

# Listen on specific port
retcat -l -p 1234
```

### TCP Client

```bash
# Connect to localhost:8080
retcat localhost 8080

# Connect to remote host
retcat example.com 80
```

### UDP Server

```bash
# Listen for UDP packets on port 8080
retcat -l -u -p 8080
```

### UDP Client

```bash
# Send UDP packets to localhost:8080
retcat -u localhost 8080
```

### Options

- `-l, --listen`: Listen mode, for inbound connections
- `-p, --port PORT`: Local port number
- `-u, --udp`: Use UDP instead of TCP
- `-h, --help`: Show help message
- `-V, --version`: Show version information

## Examples

### File Transfer

**Sender (server):**
```bash
retcat -l -p 8080 < file.txt
```

**Receiver (client):**
```bash
retcat localhost 8080 > received_file.txt
```

### Chat Between Two Machines

**Machine 1:**
```bash
retcat -l -p 8080
```

**Machine 2:**
```bash
retcat machine1-ip 8080
```

### Port Scanning

```bash
retcat -u target-host 53  # Test UDP port
retcat target-host 80     # Test TCP port
```

## Development

### Building

```bash
cargo build          # Debug build
cargo build --release # Release build
```

### Testing

```bash
cargo test
```

### Linting

```bash
cargo clippy
```

## Dependencies

- [clap](https://crates.io/crates/clap) - Command line argument parsing
- [tokio](https://crates.io/crates/tokio) - Async runtime

## License

This project is open source. See the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
