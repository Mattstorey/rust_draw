# Nannou Experiments

A collection of Rust examples exploring the [Nannou](https://nannou.cc/) creative coding framework. The examples included are just PoCs to see what Nannou can do, and showcases techniques inspired by:

- [Daniel Shiffman’s _Nature of Code_](https://github.com/nannou-org/nannou/tree/master/examples/nature_of_code)
- [Chris Biscardi’s Nannou videos](https://www.youtube.com/c/ChrisBiscardi)
- Personal experiments

## workspace

Exporation of frameworks like Nannou lends itself well to using workspaces organise the code into different experments or concepts. Using the "examples" decleative standalone executables can resides in a single `.rs` file is also helpful. This repo is laid out using Rust workspaces and uses examples in some of the workspaces. Some workspaces have a `main.rs` file. This will also allow a library of reusable parts to be added later if needed (example driven development).

## Usage

1. Install [Rust](https://www.rust-lang.org/tools/install).
2. In the project root:

To run an example:

```bash
cargo run --release --example <example_name>
```

Replace <example_name> with any of the script names in the examples workspace.

To run a `main.rs` in a workspace:

```bash
cargo run -p <workspace-name/>
```

## Future Plans

Extend these examples to drive lasers and audio. Continue exploring more creative coding concepts in Nannou.
