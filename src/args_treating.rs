use rutie::{AnyException, Object, VM};

pub trait ArgsTreating<R> {
    fn unwrap_or_rb_raise(self) -> R;
}

impl<A, E> ArgsTreating<A> for Result<A, E>
where
    A: Object,
    E: Into<AnyException> + std::fmt::Debug,
{
    fn unwrap_or_rb_raise(self) -> A {
        match self {
            Ok(object) => object,
            Err(exception) => {
                VM::raise_ex(exception);
                unreachable!();
            }
        }
    }
}
