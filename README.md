# SimTFL-Rust

SimTFL-Rust is a simulation framework for experimenting with **Best-Chain Protocols** (BC) and **Byzantine Fault Tolerance** (BFT) for the ZCash (ZEC) Trailing Finality Layer (TFL). It provides tools to model blockchains, consensus protocols, and network behavior, enabling developers to simulate and analyze complex scenarios.

---

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Running Integration Tests](#running-integration-tests)
- [Documentation](#documentation)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)

---

## Overview

SimTFL-Rust is designed to simulate the following:

- **Best-Chain Protocol (BC)**: Implements core blockchain functionality, including transactions (both shielded and un-shielded), blocks, and context management.
- **Byzantine Fault Tolerance (BFT)**: Provides a simulation framework for BFT protocols, supporting honest, Byzantine, passive, and sequential nodes.
- **Event-Driven Simulation**: Simulates message transmission, delays, and causal ordering using logical clocks and an event queue.
- **Faster-than-Real-Time Execution**: Runs simulations as fast as the host system allows, without relying on real-time delays.

The framework is modular, allowing users to extend or modify components to suit their needs.

---

## Features

- **Transaction and Block Management**: Create, validate, and manage transactions and blocks.
- **Node Types**: Supports honest, Byzantine, passive, and sequential nodes.
- **Network Simulation**: Simulates message passing, delays, and causal ordering.
- **Consensus Protocols**: Implements BFT consensus with support for proposal, voting, and finalization.
- **Extensibility**: Easily extend the framework to implement custom protocols or behaviors.

---

## Installation

To install SimTFL-Rust, ensure you have Rust and Cargo installed on your system. Then, clone the repository and build the project:

```bash
# Clone the repository
git clone https://github.com/giaki3003/simtfl-rust.git

# Navigate to the project directory
cd simtfl-rust

# Build the project
cargo build
```

---

## Running Integration Tests

SimTFL-Rust includes a suite of integration tests to verify the correctness of the simulation framework. To run the integration tests:

```bash
# Run all integration tests
cargo test --test integration
```

These tests cover key functionality, such as:

- **Node Behavior**: Tests the behavior of honest, Byzantine, and passive nodes.
- **Message Passing**: Verifies message transmission, delays, and causal ordering.
- **Consensus Protocols**: Ensures the correctness of proposal, voting, and finalization.

---

## Documentation

The documentation includes detailed descriptions of all public APIs, along with examples.

To generate local documentation:

```bash
cargo doc --workspace --open
```

---

## Examples

While SimTFL-Rust does not include a standalone demo example, you can explore the integration tests in the `tests` directory to see how the framework is used in practice. These tests demonstrate:

- **Setting up simulations** with multiple nodes.
- **Simulating message passing** and causal ordering.
- **Testing consensus protocols** under various conditions.

To run a specific integration test:

```bash
cargo test --test integration -- <test_name>
```

---

## Contributing

Contributions are welcome! To contribute:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Submit a pull request with a clear description of your changes.

For more details, see the [Contributing Guidelines](CONTRIBUTING.md).

---

## License

SimTFL-Rust is released under the [MIT License](LICENSE).

---

## Acknowledgments

- This project was (very heavily) inspired by the [Trailing Finality Layer Simulator](https://github.com/Electric-Coin-Company/simtfl) re-written from Python to Rust.
- Special thanks to the original author and to the Rust community for providing excellent tools and libraries.