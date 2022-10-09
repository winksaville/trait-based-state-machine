# trait-based-state-machine

Experiment with traits for implementing a state machine

Initially this failed but after creating a thread on discord
titled [`State Machine with traits`](https://discord.com/channels/273534239310479360/1028428961937641592)
and using [bruh![moments] two lifetime suggestion](https://discord.com/channels/273534239310479360/1028428961937641592/1028458390306947132)
it does work! This is much better than my original solution which
is in branch [return sm from process](https://github.com/winksaville/trait-based-state-machine/tree/return-sm-from-process).

After the above post, he posted a second simpler suggestion which needs only one
[explicit lifetime](https://discord.com/channels/273534239310479360/1028428961937641592/1028458436096163840).


```
$ cargo run
   Compiling trait-based-state-machine v0.2.0 (/home/wink/prgs/rust/myrepos/trait-based-state-machine)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/trait-based-state-machine`
light is off
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
