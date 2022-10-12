//#[feature(core::intrinsics)]

// Trait for processing actions in a State
pub trait State<SM> {
    fn process(&self, sm: &mut SM);
}

type StateRef<'a> = &'a dyn State<SwitchSm<'a>>;

impl PartialEq for StateRef<'_> {
    #[inline(never)]
    fn eq(&self, other: &Self) -> bool {
        //std::ptr::eq(*self, *other)
        let r = std::ptr::eq(*self, *other);
        //println!("*self={:p}", *self);
        //println!("*other={:p}", *other);
        //println!("r={}", r);
        //println!("r={r} self={self:p} *self={:p} other={other:p} *other={:p}", *self, *other);
        r
    }

    #[inline(never)]
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

// Switch state machine
struct SwitchSm<'a> {
    current_state: StateRef<'a>,
}

impl<'a> SwitchSm<'a> {
    fn new() -> Self {
        SwitchSm {
            current_state: &StateOff,
        }
    }
}

// State off
struct StateOff;

impl<'a> State<SwitchSm<'a>> for StateOff {
    #[inline(never)]
    fn process(&self, sm: &mut SwitchSm<'a>) {
        //println!("StateOff:+ sm.current_state={:p}", sm.current_state);
        sm.current_state = &StateOn;
        //println!("StateOn:  light is On");
        //println!("StateOff:- sm.current_state={:p}", sm.current_state);
    }
}

// State on
struct StateOn;

impl<'a> State<SwitchSm<'a>> for StateOn {
    #[inline(never)]
    fn process(&self, sm: &mut SwitchSm<'a>) {
        //println!("StateOn:+ sm.current_state={:p}", sm.current_state);
        sm.current_state = &StateOff;
        //println!("StateOn:  light is OFF");
        //println!("StateOn:- sm.current_state={:p}", sm.current_state);
    }
}

fn main() {
    // Create switch state machine
    //let mut switch = SwitchSm {
    //    current_state: &StateOff,
    //};
    let mut switch = Box::new(SwitchSm::new());

    // Process
    switch.current_state.process(&mut switch);
    //switch.current_state.process(&mut switch);
    //switch.current_state.process(&mut switch);

    // Validate
    if switch.current_state == &StateOff { std::process::abort(); };
    if switch.current_state != &StateOn { std::process::abort(); };
}
