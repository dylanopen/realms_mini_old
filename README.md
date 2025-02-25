# Welcome to Realms - the lightweight Rust game library

Realms is a library that allows you to easily make games,
animations and any 2D graphics using Rust.

This is a 'mid-level' crate which provides an extremely simple API to easily
draw to a graphical window, without being bloated like a typical game *engine*.
Realms is a **game library**.

## Getting started

First, create a new project with `cargo new <project_name>`.
Replace `project_name` with the name of the game you want to create.

Next, import Realms into your Rust project by adding the following line to your
**Cargo.toml**, under `[dependencies]`:

``` toml
realms = "VERSION_NUMBER"
```

Replace `VERSION_NUMBER` with the latest Realms version (displayed to the right -->).
Alternatively, run this command in your terminal from inside your project directory:

``` sh
cargo add realms
```

It is also recommended to enable **optimisations** to improve performance.
Add the following at the **end** of your `Cargo.toml`:

``` toml
[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
```

Finally, add this code to the `src/main.rs` file:

``` rust
use realms::node::{Fill, Node};
use realms::{Color, Window};

pub fn main() {
    let mut win = Window::new("Welcome to Realms", 800, 450); // create window frame
    let fill = Fill::new(Color::rgb(91, 23, 127)); // 'Fill' node - fill window blue

    while win.running {
        fill.draw(&mut win); // fill the window blue
        win.flip(); // flip the buffers to display the new background
    }
}
```

Running this code using `cargo run` will give you this result:

![An 800x600 window with title "Welcome to Realms" and filled with a purple background](docs/res/purple-window.png)

Congratulations! You have successfully written your first Realms game :\)

> Note: For more info, please see the full explanation at [github.com/dylanopen/realms](https://github.com/dylanopen/realms/tree/master/docs/examples/1-window.md)

## Documentation

The above example is just a very brief introduction to the Realms library.

Please view [the documentation on GitHub](https://github.com/dylanopen/realms/tree/master/docs)
for more info.  
There is also some information available at [docs.rs](https://docs.rs/realms),
although documentation there is limited.

Thanks for choosing Realms to build your next great game!
