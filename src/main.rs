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
