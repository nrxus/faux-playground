#![allow(clippy::new_without_default)]

pub trait A {}

#[cfg_attr(test, faux::create)]
pub struct S {}

#[cfg_attr(test, faux::methods)]
impl S {
    pub fn new() -> S {
        S {}
    }

    pub fn foo(&self, _a: &impl A) {
        todo!()
    }
}

pub fn foo() {
    struct T(u8);

    impl A for T {}

    let t = T(42);
    let s = S::new();

    s.foo(t, T(32));
}

#[cfg(test)]
#[test]
fn repro() {
    let mut mock = S::faux();
    faux::when!(mock.foo).then(|_| println!("mock foo"));
}
