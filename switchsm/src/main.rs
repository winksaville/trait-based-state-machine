//#[feature(core::intrinsics)]
use switchsm::*;

fn main() {
    println!("main:+");
    // Create switch state machine (Several different ways)

    // This works
    let switch = SwitchSm {
        current_state: &StateOff,
    };
    if switch.current_state != &StateOff { std::process::abort(); };
    if switch.current_state == &StateOff { () } else { std::process::abort(); };
    // But enabling this fails because &StateOff is different from the one set in switchsm/src/lib.rs
    //if !switch.is_state_off() { std::process::abort(); };

    // This works
    let mut switch = SwitchSm::new();
    if !switch.is_state_off() { std::process::abort(); };

    // Process
    switch.current_state.process(&mut switch);
    if !switch.is_state_on() { std::process::abort(); };

    // Process
    switch.current_state.process(&mut switch);
    if !switch.is_state_off() { std::process::abort(); };

    println!("main:-");
}
