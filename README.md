# Very minimal and dumb C++ project manager written in Rust. I took inspiration from Rust's own project manager: Cargo.

---

## Usage

cd into the project directory:

```bash
cd Castle
```

Install the binary with:

```bash
cargo install --path .
```

Make sure Cargo's binary directory is added to your `PATH`:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

After that, you can use `castle` from anywhere. It will show you the available options.

> **Note:** I do not expect this project to work on Windows machines or versions of Clang older than clang 18.
