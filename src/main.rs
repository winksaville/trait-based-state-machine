// Trait for processing actions in a State
pub trait State<SM, P> {
    fn process(&self, sm: &mut SM, msg: &P);
}

type StateRef<'a> = &'a dyn State<SwitchSm<'a>, Protocol1>;

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
        match msg {
            Protocol1::On | Protocol1::Toggle => {
                sm.current_state = &StateOn;
                println!("StateOff: light is ON");
            }
            Protocol1::Off => (),
        }
    }
}

// State on
struct StateOn;

impl<'a> State<SwitchSm<'a>, Protocol1> for StateOn {
    fn process(&self, sm: &mut SwitchSm<'a>, msg: &Protocol1) {
        match msg {
            Protocol1::Off | Protocol1::Toggle => {
                sm.current_state = &StateOff;
                println!("StateOn:  light is OFF");
            }
            Protocol1::On => (),
        }
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
}
