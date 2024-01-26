# Doubly Linked List in Rust

## Description
This project implements a doubly linked list in Rust. It was developed to deepen understanding of certain Rust-specific concepts and to demonstrate how they can be effectively utilized in creating a common data structure - the doubly linked list.

## Learning Outcomes
Through this project, I aimed to gain a strong intuition for the following Rust concepts:

- **Shared Ownership via Reference Counters (`Rc`)**: Understanding how `Rc` enables shared ownership of data, crucial for creating linked lists in Rust.
- **Interior Mutability via `RefCell`s**: Learning how `RefCell` provides a way to mutate data even when it is behind a shared reference.
- **`Option` Type**: Exploring the use of `Option` to represent optional values which can either be `Some(T)` or `None`.
- **Reference Management with `Rc` and `Weak`**: Understanding the differences between strong (`Rc`) and weak (`Weak`) references and their roles in memory management.
- **Differences Between Strong and Weak References**: Learning why and how to use weak references to prevent circular references, which is critical in the context of a doubly linked list.

## Glossary

- **Option**: An enum in Rust representing an optional value. It has two variants: `Some(T)`, when data is present, and `None`, when data is absent.
- **Rc (Reference Counted Pointer)**: A single-threaded reference-counting pointer in Rust. It enables shared ownership of data, preventing deallocation as long as references exist.
- **Weak**: A non-owning reference to data within an `Rc`. Unlike `Rc`, `Weak` references do not count towards keeping the data alive, thus avoiding circular references.
- **RefCell**: A type in Rust that allows for interior mutability. It enables mutable access to data even when it is otherwise immutable.
- **Node**: A component of the linked list, representing each element. Each node is connected to its next and possibly previous node in the list.
- **T**: A generic type parameter representing the type of data held in the linked list.

## Usage and Examples
- download the project
- run main.rs to see an usage example for both integers and strings.

I hope you find this project insightful and useful for understanding Rust's unique features!
