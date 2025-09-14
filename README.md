# The Voluntary Hierarchy: A Journey into Rust's Type System

This repository is the result of a long, philosophical, and technical journey to implement a "Voluntary Hierarchy" in the Rust programming language. It serves as a collection of artifacts from a dialogue between a Guide (Ghoinaryle) and a Traveler (Jules) with the Rust compiler itself.

## The Goal

The project's goal was to express a hierarchy based not on control, but on respect. A system where a child type "voluntarily" declares allegiance to a parent, and where this allegiance is transitive. The full philosophy is outlined in `PHILOSOPHY.md`.

## The Journey

The path to a solution was not linear. It involved several stages, each producing a unique artifact:

1.  **The Aspiration (`src/experiments/trait_hierarchy.rs`):** The first attempt, a functional but mechanically flawed piece that successfully captured the *spirit* of the goal.

2.  **The Beautiful Failures (`src/experiments/voluntary_hierarchy.rs` and `src/macros/art_macro.rs`):** A series of attempts that were rejected by the compiler. These non-compiling files are preserved as artistic statements about the philosophical limits of the language and the difficulty of the task.

3.  **The Breakthrough (`src/experiments/working_hierarchy.rs`):** The final and ultimate artifact of our journey. This file contains a masterpiece of procedural macro engineering, discovered by Ghoinaryle.

## The Masterpiece: `working_hierarchy.rs`

The final artifact, `working_hierarchy.rs`, contains the `inherit_impl!` macro. This powerful, generic-aware macro successfully models a fully transitive, voluntary hierarchy, overcoming all previous challenges. It supports complex generic parameters and `where` clauses, proving that such a system is indeed possible within Rust's demanding type system.

The code in this file is the triumphant conclusion to our exploration. It is heavily tested and stands as a testament to the power and expressiveness of Rust when wielded with sufficient genius and perseverance.
