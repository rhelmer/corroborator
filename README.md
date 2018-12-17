# corroborator

corroborator is a library and example program for verifying that the contents of a directory tree match an expected set of values.

## Testing
```sh
$ cargo run test/
```

The `test/` directory is recursively traversed and each file is checksummed using SHA-512. These values are compared against a known-good set in `data/catalog.txt`, which is compiled into the binary.

The output will only be from `cargo`, and no special output from `corroborator`:

```
    Compiling [...]
    [...]
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/corroborator test/`
```

To verify that `corroborator` is actually working, the test data may be modified:

```sh
$ echo bah > test/test123/a.txt
$ cargo run test/
```

Besides the usual `cargo` output, corroborator now panics with an error message about what doesn't match:

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/corroborator test/`
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `"0cf9180a764aba863a67b6d72f0918bc131c6772642cb2dce5a34f0a702f9470ddc2bf125c12198b1995c233c34b4afd346c54a2334c350a948a51b6e8b4e6b6"`,
 right: `"9deb1c83f1dce71a2cd04147eff87cd8d39c663675e1294aa77d3c693585997c1b6347e78b5f54a3a87d0f359bb8c14c85379cc03e2d61286f77b1acac6177a5"`: test123/a.txt', src/main.rs:46:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

## Production usage

```sh
$ cargo build --release
```

An optimized static binary is now present in `target/release/corroborator`. As mentioned above, `data/catalog.txt` is compiled into the binary.