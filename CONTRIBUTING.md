# Contributing to SimTFL-Rust

Thank you for considering contributing to SimTFL-Rust! Contributions are welcome and appreciated. Below are guidelines to help you get started.

---

## Table of Contents

- [How to Contribute](#how-to-contribute)
- [Development Workflow](#development-workflow)
- [Code Style and Formatting](#code-style-and-formatting)
- [Running Tests](#running-tests)
- [Documentation](#documentation)
- [Submit a Pull Request](#submit-a-pull-request)

---

## How to Contribute

### 1. Report Bugs
- **Check Existing Issues**: Ensure the bug hasn't already been reported.
- **Provide Details**: Include steps to reproduce the issue, expected behavior, and actual behavior.

### 2. Suggest Enhancements
- **Check Existing Issues**: Ensure the enhancement hasn't already been suggested.
- **Provide Context**: Explain the problem the enhancement solves and why it's important.

### 3. Submit Pull Requests
- Fork the repository and create a new branch for your changes.
- Ensure your changes are well-tested and documented.
- Submit a pull request with a clear description of your changes.

---

## Development Workflow

1. **Fork the Repository**: Click the "Fork" button on GitHub to create a copy of the repository.
2. **Clone Your Fork**:
   ```bash
   git clone https://github.com/your-username/simtfl-rust.git
   ```
3. **Create a New Branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```
4. **Make Your Changes**: Ensure your changes are well-documented and follow the code style guidelines.
5. **Run Tests**: Verify that your changes do not break existing functionality.
   ```bash
   cargo test
   ```
6. **Commit Your Changes**:
   ```bash
   git add .
   git commit -m "Your commit message"
   ```
7. **Push Your Changes**:
   ```bash
   git push origin feature/your-feature-name
   ```
8. **Submit a Pull Request**: Open a pull request on the main repository.

---

## Code Style and Formatting

- Follow the [Rust Code Style Guidelines](https://doc.rust-lang.org/1.0.0/style/README.html).
- Use `rustfmt` to format your code:
  ```bash
  rustfmt src/*.rs
  ```
- Ensure your code passes `clippy` checks:
  ```bash
  cargo clippy --all-targets
  ```

---

## Running Tests

To run all tests, including unit and integration tests:

```bash
cargo test
```

To run specific tests:

```bash
cargo test --test integration -- <test_name>
```

---

## Documentation

Ensure all public APIs are well-documented. Use `cargo doc` to generate documentation:

```bash
cargo doc --workspace --open
```

Provide examples in the documentation to help users understand how to use the framework.

---

## Submit a Pull Request

1. **Title**: Use a clear and descriptive title for your pull request.
2. **Description**: Provide a detailed description of your changes, including the problem they solve and how they solve it.
3. **Checklist**:
   - [ ] My changes are well-tested.
   - [ ] My changes are documented.
   - [ ] My changes follow the code style guidelines.
   - [ ] I have rebased my branch against the main branch.

---

## Additional Notes

- **Issues**: If you're unsure about a contribution, open an issue to discuss it.
- **Branch Naming**: Use descriptive branch names (e.g., `fix/bug-name`, `feature/new-feature`).
- **Commit Messages**: Write clear and concise commit messages.

Thank you for contributing to SimTFL-Rust!