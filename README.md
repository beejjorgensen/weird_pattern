Rust port of a program that shows a strange interference pattern.

From [this Reddit thread](https://www.reddit.com/r/algorithms/comments/122nhek/a_weird_fractallooking_image_that_sprouts_from/).

Generates PNGs.

Usage:

```
cargo run outfile iterations [addition]
```

If the `addition` is omitted, it will be random in range `0..=255`.

e.g.

```
cargo run foo.png 2000
cargo run foo.png 40000 10000    # will take a while!
```
