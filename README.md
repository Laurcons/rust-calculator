## simplest rust calculator
```
+---------------+
|             0 |
+---------------+
|  ON     C  CE |
|  1   2   3  * |
|  4   5   6  / |
|  7   8   9    |
|  0   +   -  = |
+---------------+
> q to exit     <
> d for debug   <
```

A calculator implementation made as an exercise to learn Rust. It kinda works, I had fun learning how to implement traits and whatnot.

I tried to make the code as readable and organized as possible. There are `.unwrap()`s here and there, but they're for functionality without which the program wouldn't work anyway.

The `controller` module is the brains of the calculator, while the `view` module will print the calculator in the console, and listen for keypresses.

## running this
Have rustup and cargo installed.

Run `cargo run` to build and run.