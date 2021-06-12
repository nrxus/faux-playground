use async_trait::async_trait;

#[async_trait]
pub trait AsyncTrait {
    async fn my_method(&self) -> i32;
}

#[cfg_attr(test, faux::create)]
pub struct MyStruct;

#[cfg_attr(test, faux::methods)]
#[async_trait]
impl AsyncTrait for MyStruct {
    async fn my_method(&self) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use faux::when;

    #[test]
    fn it_works() {
        let mut faked = MyStruct::faux();
        when!(faked.my_method).then(|_| 3);
        let fetched = futures::executor::block_on(faked.my_method());
        assert_eq!(fetched, 3);
    }
}
