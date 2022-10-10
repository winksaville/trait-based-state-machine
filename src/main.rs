// Trait for processing actions in a State
pub trait State<SM, P> {
    fn process(&self, sm: &mut SM, msg: &P);
}

type StateRef<'a> = &'a dyn State<SwitchSm<'a>, Protocol1>;

#[allow(unused)]
enum Protocol1 {
    On,
    Off,
    Toggle,
}

// Switch state machine
struct SwitchSm<'a> {
    current_state: StateRef<'a>,
    light_on: bool,
}

// State off
struct StateOff;

impl<'a> State<SwitchSm<'a>, Protocol1> for StateOff {
    fn process(&self, sm: &mut SwitchSm<'a>, msg: &Protocol1) {
        match msg {
            Protocol1::On | Protocol1::Toggle => sm.light_on = true,
            Protocol1::Off => (),
        }
        println!("StateOff: light_on is {}", sm.light_on);
    }
}

fn main() {
    // Create state off
    let state_off = StateOff;

    // Create switch state machine
    let mut switch = SwitchSm {
        current_state: &state_off,
        light_on: false,
    };

    // Create Message
    let msg = Protocol1::On;

    // Process
    switch.current_state.process(&mut switch, &msg);

    // Validate
    assert!(switch.light_on);
}
