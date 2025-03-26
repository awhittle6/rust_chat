# Rust gRPC Bidirectional Chat Application

This is a simple Rust-based gRPC bidirectional chat application using [`tonic`](https://github.com/hyperium/tonic). It includes a server and a client implementation that communicate using Protocol Buffers over gRPC.

## Table of Contents
- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installing Rust and Cargo](#installing-rust-and-cargo)
- [Setup](#setup)
- [Compiling Protobuf Files](#compiling-protobuf-files)
- [Running the Application](#running-the-application)
- [Project Structure](#project-structure)
- [License](#license)

## Features

- Bidirectional streaming between client and server
- Simple, terminal-based interface for chatting
- Built using Rust and `tonic`

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Protobuf Compiler (`protoc`)](https://grpc.io/docs/protoc-installation/)

## Installing Rust and Cargo

```bash
# Install Rust and Cargo (includes rustup)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Ensure rust and cargo are installed correctly
rustc --version
cargo --version
```

## Setup

Clone this repository and navigate into the project directory:

```bash
git clone https://github.com/your-username/rust-grpc-chat.git
cd rust-grpc-chat
```

Install dependencies:

```bash
cargo build
```

## Compiling Protobuf Files

Ensure you have `protoc` installed, then compile the `.proto` files:

```bash
# Install protoc if you haven't
# macOS: brew install protobuf
# Ubuntu: sudo apt install -y protobuf-compiler

# Run build script to compile .proto files
cargo build
```

This will generate the Rust code for the `.proto` files into the appropriate directory (usually under `src/generated` or `OUT_DIR` depending on build.rs).

## Running the Application

### Start the Server

```bash
cargo run --quiet --bin grpc-server
```

### Start the Client

Open another terminal window and run:

```bash
cargo run --quiet --bin grpc-client
```

Now you can start chatting between client and server using bidirectional gRPC streams.

## Project Structure

```
.
├── Cargo.toml
├── build.rs
├── proto/
│   └── chat.proto
├── src/
│   ├── bin/
│   │   ├── grpc-client.rs
│   │   └── grpc-server.rs
│   ├── generated/       # Auto-generated gRPC files
│   └── lib.rs
```

## License

This project is licensed under the MIT License.