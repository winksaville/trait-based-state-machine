// This is an attempt to implement bruh![moment] suggestion to use std::any::Any
// but I'm unable to get it to compile. I'm probably approaching it totally wrong.
// See: https://discord.com/channels/273534239310479360/1028428961937641592/1029167534097834044
//
// Also adding as_any and equals would seem to extra dynamic
// dispatching and if my use of std::ptr::eq is fragile then
// it seems better to go back to using SwitchSm::light_on. I
// suspect this would be more preformant then the extra dynamic
// dispatching.

// Trait for processing actions in a State
pub trait State<SM, P> {
    fn as_any(&self) -> &dyn std::any::Any;
    fn equals(&self, _other: &dyn State<SM, P>) -> bool;
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
#[derive(PartialEq)]
struct StateOff;

impl<'a> State<SwitchSm<'a>, Protocol1> for StateOff {
    //-----
    // Having to duplicate this code here and in StateOn isn't good
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn equals(&self, other: &dyn State<SwitchSm<'a>, Protocol1>) -> bool {
        let o_any = other.as_any();
        let o_downcast_ref = o_any.downcast_ref::<&dyn State<SwitchSm<'a>, Protocol1>>();
                                                    // compile error ^^^^^^^^^^^^ requires that `'a` must outlive `'static`

        let s_any = self.as_any();
        let s_downcast_ref = s_any.downcast_ref::<&dyn State<SwitchSm<'a>, Protocol1>>();

        let r = if let Some(o) = o_downcast_ref {
            if let Some(s) = s_downcast_ref {
                s == o
            } else {
                false
            }
        } else {
            false
        };

        //let r = o_downcast_ref.map_or(false, |o| self == *o);

        r
    }
    //-----

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
    //-----
    // Having to duplicate this code here and in StateOff isn't good
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn equals(&self, other: &dyn State<SwitchSm<'a>, Protocol1>) -> bool {
        let o_any = other.as_any();
        let o_downcast_ref = o_any.downcast_ref::<&dyn State<SwitchSm<'a>, Protocol1>>();
                                                    // compile error ^^^^^^^^^^^^ requires that `'a` must outlive `'static`

        let s_any = self.as_any();
        let s_downcast_ref = s_any.downcast_ref::<&dyn State<SwitchSm<'a>, Protocol1>>();

        let r = if let Some(o) = o_downcast_ref {
            if let Some(s) = s_downcast_ref {
                s == o
            } else {
                false
            }
        } else {
            false
        };

        //let r = o_downcast_ref.map_or(false, |o| self == *o);

        r
    }
    //-----

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
