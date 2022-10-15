use ::state::{StateRef, State};

// Switch state machine
pub struct SwitchSm<'a> {
    pub current_state: StateRef<'a, Self>,
}

impl<'a> SwitchSm<'a> {
    //#[inline(never)]
    pub fn new() -> Self {
        SwitchSm {
            current_state: &StateOff,
        }
    }

    //#[inline(never)]
    pub fn is_state_on(&self) -> bool {
        self.current_state == &StateOn
    }

    //#[inline(never)]
    pub fn is_state_off(&self) -> bool {
        self.current_state == &StateOff
    }
}

// State off
pub struct StateOff;

impl<'a> State<SwitchSm<'a>> for StateOff {
    //#[inline(never)]
    fn process(&self, sm: &mut SwitchSm<'a>) {
        //println!("StateOff:+ sm.current_state={:p}", sm.current_state);
        sm.current_state = &StateOn;
        //println!("StateOn:  light is On");
        //println!("StateOff:- sm.current_state={:p}", sm.current_state);
    }
}

// State on
pub struct StateOn;

impl<'a> State<SwitchSm<'a>> for StateOn {
    //#[inline(never)]
    fn process(&self, sm: &mut SwitchSm<'a>) {
        //println!("StateOn:+ sm.current_state={:p}", sm.current_state);
        sm.current_state = &StateOff;
        //println!("StateOn:  light is OFF");
        //println!("StateOn:- sm.current_state={:p}", sm.current_state);
    }
}
