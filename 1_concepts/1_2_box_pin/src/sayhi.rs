use std::rc::Rc;
use std::pin::Pin;
use std::fmt;



trait MyTrait: fmt::Debug {}

pub trait SayHi: fmt::Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self);
    }
}



#[derive(Debug)]
struct MyType;
impl MyTrait for MyType {}




impl<T: MyTrait> SayHi for Box<T> {}
impl<T: MyTrait> SayHi for Rc<T> {}
impl<T: MyTrait> SayHi for Vec<T> {}
impl SayHi for String {}
impl SayHi for &[u8] {}



#[allow(non_snake_case)]
mod impl_T {
    use std::pin::Pin;
    use std::fmt::Debug;

    
    trait MyTrait: Debug {}
    trait SayHi: std::fmt::Debug {
        fn say_hi(self: Pin<&Self>) {
            println!("Hi from {:?}", self);
        }
    }
    
    
    #[derive(Debug)]
    struct MyType;
    impl MyTrait for MyType {}
    

    impl<T: MyTrait> SayHi for T {}
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_sayhi() {
        let boxed = Box::new(MyType);
        let rced = Rc::new(MyType);
        let veced = vec![String::from("dkfdjfkd")];
        let strhere = String::from("dfkjsldjkfsdf");

        let u8val:[u8; 3] = [8, 2, 1];
        let u8here = &u8val[..];

        println!("{:?}", boxed);
        println!("{:?}", rced);
        println!("{:?}", veced);
        println!("{:?}", strhere);
        println!("{:?}", u8here);
    }
}