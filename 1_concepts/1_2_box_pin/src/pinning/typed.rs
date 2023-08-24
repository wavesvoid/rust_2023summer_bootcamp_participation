/*! THE WHOLE POINT IS THAT THERE IS NO POINT.
 * 
 * This module provides different implementations of some trait 
 * which takes Pin<&mut Self> as self.
 * 
 * There is no practical application for this type of implementations
 * other than to practice using Pin methods.
 * Safe Pin does not make sense.
 */

use std::pin::Pin;
use std::rc::Rc;



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


/* *****
 * BOX
 * ******* */
impl<T: From<u16>> MutMeSomehow for Box<T> {
    fn mut_me_somehow(mut self: Pin<&mut Self>) {
        self.set(Box::new(T::from(32)));
    }
}


/* *****
 * RC
 * ******* */
impl<T: From<String>> MutMeSomehow for Rc<T> {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let ptr = self.get_mut();
        *ptr = Rc::new(T::from("kdjfkdj".to_owned()));
    }
}


/* *****
 * Vec
 * ******* */
 impl<T: From<i32>> MutMeSomehow for Vec<T> {
    fn mut_me_somehow(mut self: Pin<&mut Self>) {
        self.set(vec![T::from(64)]);
    }
}


/* *****
 * STRING
 * ******* */
impl MutMeSomehow for String {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        // Obtaining a direct pointer to the underlying Unpin type
        *Pin::into_inner(self) = String::from("Empty string");
    }
}


/* *****
 * &[u8]
 * ******* */
impl MutMeSomehow for &[u8] {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let item = self.get_mut();
        const ITEM_NEW: [u8; 3] = [1, 2, 3];
        std::mem::swap(item, &mut &ITEM_NEW[..]);
    }
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_pinbox() {
        let mut box_ptr = Box::new(324 as usize);
        let assertion_type = usize::from(32 as u8);

        let box_pin = Pin::new(&mut box_ptr);
        box_pin.mut_me_somehow();

        assert_eq!(*box_ptr, assertion_type);
    }

    #[test]
    fn test_pinrc() {
        let mut rc_ptr = Rc::new(String::from("customstr"));
        let assertion_type = String::from("kdjfkdj");

        let pinrc = Pin::new(&mut rc_ptr);
        pinrc.mut_me_somehow();

        assert_eq!(*rc_ptr, assertion_type);
    }

    #[test]
    fn test_pinvec() {
        let mut vec_ptr = vec![72.0 as f64];
        let assertion_type = vec![64.0 as f64];

        let pinvec = Pin::new(&mut vec_ptr);
        pinvec.mut_me_somehow();

        assert_eq!(*vec_ptr, assertion_type);
    }

    #[test]
    fn test_pinstring() {
        let mut string_ptr = String::from("BestTvarynka");
        let assertion_type = String::from("Empty string");

        let pinstring = Pin::new(&mut string_ptr);
        pinstring.mut_me_somehow();

        assert_eq!(string_ptr, assertion_type);
    }

    #[test]
    fn test_pinu8() {
        let u8val = [34, 23, 123, 129];
        let mut u8ptrs = &u8val[..];

        let assertion_type = [1, 2, 3] as [u8; 3];

        let pinu8 = Pin::new(&mut u8ptrs);
        pinu8.mut_me_somehow();
        

        assert_eq!(u8ptrs, assertion_type);
    }
}