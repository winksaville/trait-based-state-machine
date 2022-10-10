// Trait for processing actions in a State
pub trait State<SM> {
    fn process(&self, sm: &mut SM);
}

type StateRef<'a> = &'a dyn State<SwitchSm<'a>>;

// Switch state machine
struct SwitchSm<'a> {
    current_state: StateRef<'a>,
    light_on: bool,
}

// State off
struct StateOff;

impl<'a> State<SwitchSm<'a>> for StateOff {
    fn process(&self, sm: &mut SwitchSm<'a>) {
        sm.light_on = false;
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

    // Process
    switch.current_state.process(&mut switch);

    // Validate
    assert!(!switch.light_on);
}
