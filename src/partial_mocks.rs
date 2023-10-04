#[cfg_attr(test, faux::create)]
pub struct Foo {
    a: i32,
    b: u64,
}

#[cfg_attr(test, faux::methods)]
impl Foo {
    pub fn new(a: i32, b: u64) -> Foo {
        Foo { a, b }
    }

    pub fn to_be_mocked(&self) -> i32 {
        self.a
    }

    pub fn to_be_real(&self) -> u64 {
        self.b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial() {
        let mut mock = Foo::faux();
        let real = Foo::new(2, 3);
        faux::when!(mock.to_be_mocked()).then_return(999);
        faux::when!(mock.to_be_real()).then(move |_| real.to_be_real());

        assert_eq!(mock.to_be_mocked(), 999);
        assert_eq!(mock.to_be_real(), 3);
    }
}
