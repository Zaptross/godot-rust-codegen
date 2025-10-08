# godot-rust-codegen

godot-rust-codegen is my set of opinionated codegen tools for working with [Godot](https://godotengine.org/) using [godot-rust](https://godot-rust.github.io/).

These tools are built from my workflows while working with a few godot-rust projects.

## Usage

You can use these tools by adding the following to your `Cargo.toml`:

```toml
[build-dependencies]
zgrcg = { git = "https://github.com/zaptross/godot-rust-codegen", branch = "main" }
```

Then configure your generation settings in your `build.rs`. It's recommended to run the generator as early as is possible in `build.rs` to ensure smooth builds.

### Simple layer constant generation configuration example:

```rust
// build.rs
use zgrcg::Generator;

fn main() {
    Generator::builder()
        // tell the generator where to output code
        .set_output_dir("./src/generated".into())
        // tell the generator where to find the project.godot file as it is the
        // source of truth for layer definitions.
        .set_project_godot_path("./project.godot".into())
        // enable layer const generation
        .output_layer_consts()
        // run the generator
        .generate();

    // ... rest of build process
}
```

Configured as above, the generator will find the `[layers]` section of your `project.godot` file:

```
[layer_names]

2d_physics/layer_1="collisions"
2d_physics/layer_2="noncolliding"
2d_render/layer_1="ghosts"
```

And will generate an enum for each layer group, like:

```rust
#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Physics2d {
    COLLISIONS = 1,
    NONCOLLIDING = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Render2d {
    GHOSTS = 1,
}
```

See [the example `build.rs`](./example/build.rs) for a full configuration, and run it with `make example` (or `cd example && cargo build`) to see the output files and changes.

## Features and Configuration

|Feature|Description|Requires configuration|Example|
|-|-|-|-|
|Layer Consts|Generates enums grouped by layer|`set_output_dir`,`set_project_godot_path`|[layers.rs](./example/src/generated/layers.rs)|
|Action Consts|Generates const-like functions for each action|`set_output_dir`,`set_project_godot_path`|[action_consts.rs](./example/src/generated/actions_consts.rs)|
|Action Invocations|Generates extension functions for godot's input singleton|`set_output_dir`,`set_project_godot_path`|[action_invocations.rs](./example/src/generated/actions_invocations.rs)|
|Icon Comments*|Manages custom class icons in `.gdextension` file from icon source|`set_output_dir`, `set_resource_path`, `set_gdextension_path`, `add_icon_source`|[rust.gdextension](./example/rust.gdextension)|

**\*** This procedure creates a backup, _then_ modifies your `.gdexension` file to add icon declarations. Note: the backup created this way will not be overwritten by this process, to ensure at least one good copy of the .gdextension file exists.

** Rust files generated this way will be linked together in a `mod.rs` at the specified output directory. E.g. [mod.rs](./example/src/generated/mod.rs)