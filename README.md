# Doubly Linked List in Rust

## Description
This project implements a doubly linked list in Rust, aimed at deepening understanding of specific Rust concepts and demonstrating their application in creating a common data structure.

## Learning Outcomes
The project focuses on:

- **Shared Ownership via `Rc`**: Demonstrating how `Rc` enables shared data ownership, essential for linked lists in Rust.
- **Interior Mutability via `RefCell`s**: Utilizing `RefCell` for mutable data access in an immutable context.
- **`Option` Type**: Employing `Option` to handle optional values (`Some(T)` or `None`).
- **Reference Management with `Rc` and `Weak`**: Differentiating strong (`Rc`) and weak (`Weak`) references for effective memory management.
- **Strong vs. Weak References**: Using weak references to prevent circular references in the doubly linked list.

## Glossary

- **Option**: An enum representing an optional value, either `Some(T)` or `None`.
- **Rc**: A single-threaded reference-counting pointer. It enables shared data ownership.
- **Weak**: A non-owning reference type used in conjunction with `Rc`.
- **RefCell**: A type for interior mutability, allowing mutable data access in otherwise immutable contexts.
- **Node**: A linked list element, connected to its subsequent and potentially previous node.
- **T**: A generic type parameter, indicating the data type held in the linked list.

## Usage and Examples
To explore and test the doubly linked list:

- **Running Examples**:
  - Download the project.
  - Use `cargo run` to execute `main.rs`, which contains examples of the linked list being used with both integers and strings.

- **Running Unit Tests**:
  - The `doubly.rs` file includes a suite of unit tests.
  - Use `cargo test` to run these tests and verify the functionality of the linked list implementation.

These steps will help you understand the implementation and ensure its reliability through testing.

I hope you find this project helpful in understanding Rust's unique features!

