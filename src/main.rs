// Trait for processing actions in current State
pub trait State<'a, SM> {
    fn process(&self, sm: &'a mut SM);
}

// Switch state machine
struct SwitchSm<'a> {
    current_state: &'a dyn State<'a, SwitchSm<'a>>,
    light_on: bool,
}

// State off
struct StateOff;

impl<'a> State<'a, SwitchSm<'a>> for StateOff {
    fn process(&self, sm: &'a mut SwitchSm<'a>) {
        sm.light_on = false;
        println!("light is off");
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

    // Enabling assert! causes error[E0503]: cannot use `switch.light_on` because it was mutably borrowed
    assert!(switch.light_on == false);
}
