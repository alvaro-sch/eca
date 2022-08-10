# eca

[Elementary Cellular Automaton](https://en.wikipedia.org/wiki/Elementary_cellular_automaton)
visualizer, writen in C by [leo10torres](https://github.com/leo10torres)
and me, rewritten in Rust using the [image](https://docs.rs/image/latest/image/)
crate.

## Running the program

```sh
cargo run --release -- width height scale rule [filename]
```

### Example run

```sh
cargo run --release -- 1920 1080 1 18
```

Will output [Sierpiński triangle](https://en.wikipedia.org/wiki/Sierpi%C5%84ski_triangle)
in `./18.png`.
