# corroborator

corroborator is a library and example program for verifying that the contents of a directory tree match an expected set of values.

## Testing
```sh
$ cargo run ./test
```

The `test/` directory is recursively traversed and each file is checksummed using SHA-512. These values are compared against a known-good set in `data/catalog.txt`, which is compiled into the binary.

The output will only be from `cargo`, and no special output from `corroborator`:

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/corroborator test/`
```

This is to match the behavior of the Unix `diff` command.

To verify that `corraborator` is actual working, the test data may be modified:

```sh
$ echo bah > test/test123/a.txt
$ cargo run ./test
```

Besides the usual `cargo` output, a human-readable diff is produced:

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/corroborator test/`
 72f25d90ef4cfecda8fa2c47561af5af0a10a92bfd15986b1f916358bf6ac8a37858a14d27329506a3766bad0f34d2e04caf397c1607b4380eb33c97d37dfc37 test123/c.txt
 cc06808cbbee0510331aa97974132e8dc296aeb795be229d064bae784b0a87a5cf4281d82e8c99271b75db2148f08a026c1a60ed9cabdb8cac6d24242dac4063 test123/b.txt
-9deb1c83f1dce71a2cd04147eff87cd8d39c663675e1294aa77d3c693585997c1b6347e78b5f54a3a87d0f359bb8c14c85379cc03e2d61286f77b1acac6177a5 test123/a.txt
+0cf9180a764aba863a67b6d72f0918bc131c6772642cb2dce5a34f0a702f9470ddc2bf125c12198b1995c233c34b4afd346c54a2334c350a948a51b6e8b4e6b6 test123/a.txt

thread 'main' panicked at 'does not match', src/main.rs:45:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

## Production usage

```sh
$ cargo build --release
```

An optimized static binary is now present in `./target/release/corroborator`. As mentioned above, `./data/catalog.txt` is compiled into the binary.