# Binary Trees in Rust

BST · AVL · Red-Black Tree

---

## Overview

This project implements three classical binary tree data structures in Rust:

* Binary Search Tree (BST)
* AVL Tree
* Red-Black Tree (RBT)

The goal is to provide clear and faithful implementations of each structure, preserving their original algorithms while adapting them to Rust’s ownership and safety model.

The code emphasizes:

* clarity over abstraction
* explicit algorithmic steps
* implementation entirely in safe Rust (no `unsafe` blocks)
* close correspondence with traditional textbook approaches

---

## Project Structure

src/
└── bin/
  ├── bst/
  │  ├── main.rs
  │  └── bst.rs
  ├── avl/
  │  ├── main.rs
  │  └── avl.rs
  └── rbt/
    ├── main.rs
    └── rbt.rs

Each tree is implemented as a separate binary target.

* `main.rs` handles the command-line interface
* `bst.rs`, `avl.rs`, and `rbt.rs` contain the data structure logic

---

## Implemented Structures

### 1. Binary Search Tree (BST)

A standard binary search tree without self-balancing.

Properties:

* Left subtree contains values smaller than the node
* Right subtree contains values greater than the node
* Duplicate values are not inserted

Operations:

* Iterative insertion
* Search
* Removal using the in-order successor
* Height calculation
* In-order traversal
* Level-order visualization

Complexity:

| Operation | Average  | Worst |
| --------- | -------- | ----- |
| Search    | O(log n) | O(n)  |
| Insert    | O(log n) | O(n)  |
| Remove    | O(log n) | O(n)  |

---

### 2. AVL Tree

A self-balancing binary search tree.

Each node stores its height and maintains a balance factor defined as:

balance = height(left) − height(right)

Valid values are −1, 0, and 1. Any imbalance triggers rotations.

Rotations:

* Left rotation
* Right rotation
* Left-Right rotation
* Right-Left rotation

Operations:

* Recursive insertion with rebalancing
* Recursive removal with rebalancing
* Search
* Height calculation
* Traversal and visualization

Complexity:

| Operation | Time     |
| --------- | -------- |
| Search    | O(log n) |
| Insert    | O(log n) |
| Remove    | O(log n) |

---

### 3. Red-Black Tree (RBT)

A balanced binary search tree based on node coloring.

Properties:

1. Each node is either red or black
2. The root is always black
3. Red nodes cannot have red children
4. Every path from a node to its descendant leaves contains the same number of black nodes

Implementation approach:

Instead of pointer-based references, this implementation uses a vector of nodes with index-based links.

Each node stores:

* parent index
* left child index
* right child index

A missing child is represented as `None`, which behaves as a black NIL node.

This approach avoids:

* unsafe code
* borrowing conflicts
* complex lifetime management

Operations:

* Insertion with fix-up (recoloring and rotations)
* Removal with fix-up (case-based balancing)
* Search
* Height calculation
* Traversals

Complexity:

| Operation | Time     |
| --------- | -------- |
| Search    | O(log n) |
| Insert    | O(log n) |
| Remove    | O(log n) |

---

## Public API Behavior

Core operations return boolean values when appropriate:

* `insert(value)` returns true if the value was inserted, false if it already exists
* `search(value)` returns true if the value is found
* `remove(value)` returns true if the value was removed, false if it does not exist

This design keeps the data structure logic independent from user interaction, allowing the CLI layer to handle all output messages.

---

## Design Decisions

1. **Safe Rust only**
   All implementations avoid the use of `unsafe`.

2. **Explicit ownership model**

   * BST and AVL use `Box<Node>`
   * RBT uses `Vec<Node>` with index-based references

3. **Conservative implementation style**
   The algorithms follow a step-by-step structure rather than relying on abstractions. This makes them easier to compare with implementations in other languages such as C or Java.

4. **Separation of concerns**

   * Data structure logic is isolated in `*.rs` files
   * User interaction is handled in `main.rs`

---

## How to Run

From the project root:

cargo run --bin bst
cargo run --bin avl
cargo run --bin rbt

---

## CLI Features

Each executable provides:

* Insert value
* Search value
* Remove value
* Calculate tree height
* Print tree (in-order)
* Print tree (level view)

---

## Output Representation

In-order traversal prints values in sorted order.

Example:
1 3 5 7 9

Level-order view displays the structure of the tree using spacing and placeholders (`n` or `---`).

The Red-Black Tree also displays node colors.

Example:
10[B] 15[R] 20[B]

---

## Testing

The test suite covers:

* insertion and search
* duplicate handling
* removal of existing and missing values
* height calculation for empty trees
* AVL balancing under ordered insertions

Tests are implemented for all three structures (BST, AVL, and RBT).

---

## Educational Notes

This project is intended as a study reference.

It highlights:

* how recursion interacts with ownership in Rust
* how rotations restructure binary trees
* how AVL and Red-Black Trees maintain balance differently
* how safe Rust can represent pointer-like structures

---

## Author

Amauri B. S. Junior

---

## License

This project is provided for educational purposes.
