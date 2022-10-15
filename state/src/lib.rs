// Trait for processing actions in a State
pub trait State<SM> {
    fn process(&self, sm: &mut SM);
}

pub type StateRef<'a, SM> = &'a dyn State<SM>;

impl<SM> PartialEq for StateRef<'_, SM> {
    //#[inline(never)]
    fn eq(&self, other: &Self) -> bool {
        //std::ptr::eq(*self, *other)
        //let r = std::ptr::eq(*self, *other);
        //println!("r={r} self={self:p} *self={:p} other={other:p} *other={:p}", *self, *other);
        //r
        std::ptr::eq(*self, *other)
    }

    //#[inline(never)]
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
