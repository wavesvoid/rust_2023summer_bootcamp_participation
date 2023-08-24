use std::pin::Pin;



pub trait MutMeSomehow {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        // Implementation must be meaningful, and
        // obviously call something requiring `&mut self`.
        // The point here is to practice dealing with
        // `Pin<&mut Self>` -> `&mut self` conversion
        // in different contexts, without introducing 
        // any `Unpin` trait bounds.
    }
}


impl<T: Default> MutMeSomehow for T
{
    fn mut_me_somehow(mut self: Pin<&mut T>) {
        self.set(T::default());
    }
}





#[cfg(test)]
pub mod testing {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct MyType(i32);
    impl Default for MyType {
        fn default() -> Self {
            Self(6)
        }
    }


    #[test]
    fn test_pin_generic() {
        let mut data1 = MyType(12);
        let mut data2 = String::from("dfjkdjfk");

        let assertion_type1 = MyType(6);
        let assertion_type2 = String::new();

        let pini32 = Pin::new(&mut data1);
        let pinstr = Pin::new(&mut data2);

        pini32.mut_me_somehow();
        pinstr.mut_me_somehow();
        
        assert_eq!(data1, assertion_type1);
        assert_eq!(data2, assertion_type2);
    }
    
}
