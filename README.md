# trait-based-state-machine

Experiment with traits for implementing a state machine

Initially this failed but after creating a thread on discord
titled [`State Machine with traits`](https://discord.com/channels/273534239310479360/1028428961937641592)
and using [bruh![moments] two lifetime suggestion](https://discord.com/channels/273534239310479360/1028428961937641592/1028458390306947132)
it does work! This is much better than my original solution which
is in branch [return sm from process](https://github.com/winksaville/trait-based-state-machine/tree/return-sm-from-process).

After the above post, bruh![moments] posted a second simpler suggestion which needs only one
[explicit lifetime](https://discord.com/channels/273534239310479360/1028428961937641592/1028458436096163840).

I've now created a more complete state machine, it actually has two
states and also processes On, Off and Toggle messages. I also
removed `SwitchSm::light_on` and just use `SwitchSm::current_state` to
know if the light is on or off. I did run into a problem with the `PartialEq`
code. And again [bruh![moments]]():
```
// Trait for processing actions in a State
pub trait State<SM, P> {
    fn process(&self, sm: &mut SM, msg: &P);
}

type StateRef<'a> = &'a dyn State<SwitchSm<'a>, Protocol1>;

impl PartialEq for StateRef<'_> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(*self, *other)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for StateRef<'_> {}

enum Protocol1 {
    On,
    Off,
    Toggle,
}

// Switch state machine
struct SwitchSm<'a> {
    current_state: StateRef<'a>,
}

// State off
struct StateOff;

impl<'a> State<SwitchSm<'a>, Protocol1> for StateOff {
    fn process(&self, sm: &mut SwitchSm<'a>, msg: &Protocol1) {
        //println!("StateOff:+ sm.current_state={:p}", sm.current_state);
        match msg {
            Protocol1::On | Protocol1::Toggle => {
                sm.current_state = &StateOn;
                println!("StateOff: light is ON");
            }
            Protocol1::Off => (),
        }
        //println!("StateOff:- sm.current_state={:p}", sm.current_state);
    }
}

// State on
struct StateOn;

impl<'a> State<SwitchSm<'a>, Protocol1> for StateOn {
    fn process(&self, sm: &mut SwitchSm<'a>, msg: &Protocol1) {
        //println!("StateOn:+ sm.current_state={:p}", sm.current_state);
        match msg {
            Protocol1::Off | Protocol1::Toggle => {
                sm.current_state = &StateOff;
                println!("StateOn:  light is OFF");
            }
            Protocol1::On => (),
        }
        //println!("StateOn:- sm.current_state={:p}", sm.current_state);
    }
}

fn main() {
    // Create switch state machine
    let mut switch = SwitchSm {
        current_state: &StateOff,
    };

    // Create Messages
    let msg_off = Protocol1::Off;
    let msg_on = Protocol1::On;
    let msg_toggle = Protocol1::Toggle;

    // Process
    switch.current_state.process(&mut switch, &msg_on);
    switch.current_state.process(&mut switch, &msg_off);
    switch.current_state.process(&mut switch, &msg_toggle);

    // Validate
    assert!(switch.current_state == &StateOn);
    assert!(switch.current_state != &StateOff);
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
