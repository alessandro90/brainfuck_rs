# Brainfuck

Very small interpreter of the [brainfuck](https://esolangs.org/wiki/Brainfuck) programming language written in Rust.

It can parse a file or run as a repl.

## Notes

- Each cell is an `u8` with wrapping behaviour.
- The underlying buffer of cells is dynamic. No upper limit.
- No language extensions.

## Usage

### Using a file

```shell
cargo run hello_world.bf
```

The command above will open and interpret the file _hello\_world.bf_.

## REPL

```shell
cargo run
```

The command above will start the repl.

Each line provided to the repl will be immediately executed after pressing _enter_ unless the line ends with a semicolon. In such a case the repl waits for another line to be provided before executing the program. For example

```shell
$ ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>;
$ ->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.;
$ +++.------.--------.>>+.>++.
Hello world!
```

The three lines above are executed as a single string.
