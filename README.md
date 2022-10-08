# trait-based-state-machine

Experiment with traits for implementing a state machine

This is currently failing with:

```
$ cargo run
   Compiling expr-traits-1 v0.1.0 (/home/wink/prgs/rust/myrepos/exper-traits-1)
error[E0503]: cannot use `switch.light_on` because it was mutably borrowed
  --> src/main.rs:36:13
   |
33 |     switch.current_state.process(&mut switch);
   |                                  ----------- borrow of `switch` occurs here
...
36 |     assert!(switch.light_on == false);
   |             ^^^^^^^^^^^^^^^
   |             |
   |             use of borrowed `switch`
   |             borrow later used here

For more information about this error, try `rustc --explain E0503`.
error: could not compile `expr-traits-1` due to previous error
```

I can get it working by having State::Process return &'a mut SM, see
https://github.com/winksaville/trait-based-state-machine/tree/return-sm-from-process

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
