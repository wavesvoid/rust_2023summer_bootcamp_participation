use std::cell::RefCell;
use std::rc::Rc;
// use std::ops::{Deref, DerefMut};


/// A simple Error kind for a single case
#[derive(PartialEq, Debug)]
pub enum GSError {
    PushError,
}

#[derive(Clone)]
pub struct GlobalStack<T> {
    stack: Rc<RefCell<Vec<T>>>
}

impl<T: Clone> Default for GlobalStack<T> {
    fn default() -> Self {
        Self::new()
    }
}


impl<T: Clone> GlobalStack<T> {
    pub fn new() -> Self {
        Self {
            stack: Rc::new(RefCell::new(vec![]))
        }
    }

    /// Put some data T inside the GlobalStack
    pub fn push(&mut self, value: T) {
        self.stack.as_ref().borrow_mut().push(value)
    }

    /// Pop T from the GlobalStack
    pub fn pop(&self) -> Option<T> {
        self.stack.as_ref().borrow_mut().pop()
    }

    /// Receive a copy of the last T
    pub fn clone_tail(&self) -> Option<T> {
        let s = self.stack.as_ref().borrow();
        Some((*s.get(0)?).clone())
    }

    /// Performe complete copy of the internal stack and return this copy
    pub fn reciprocate_clone(&self) -> Vec<T> {
        (*self.stack).borrow().clone()
    }
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_data_share_validity() {
        let mut shs = GlobalStack::<String>::new();
        let another_stack_ref = shs.clone();
        let third_stack_ref = shs.clone();
        
        
        shs.push(String::new());


        // Test tails
        assert_eq!(shs.clone_tail().unwrap(),
                   another_stack_ref.clone_tail().unwrap());
        assert_eq!(another_stack_ref.clone_tail().unwrap(),
                   third_stack_ref.clone_tail().unwrap());
        
        // Test whole data contained withing internal structure
        assert_eq!(shs.reciprocate_clone(),
                   another_stack_ref.reciprocate_clone());
        assert_eq!(another_stack_ref.reciprocate_clone(),
                   third_stack_ref.reciprocate_clone());
    }

    #[test]
    fn test_access_methods() {
        let mut shs = GlobalStack::<String>::new();

        let s1 = String::from("heymay");
        let s2 = String::from("heymay2");
        let s3 = String::from("heymay3");

        // Push some data to the stack
        shs.push(s1.clone());
        shs.push(s2.clone());
        shs.push(s3.clone());

        // Confirm whole data is on the stack
        assert_eq!(shs.reciprocate_clone(), 
                   vec![s1.clone(),
                        s2.clone(),
                        s3.clone(), ]);


        // Extract from stack one-by-one
        let last_1 = shs.pop();
        let last_2 = shs.pop();
        let last_3 = shs.pop();

        // Verify one-by-one extracted data
        assert_eq!(last_1, Some(s3));
        assert_eq!(last_2, Some(s2));
        assert_eq!(last_3, Some(s1));
    }

    #[test]
    fn test_different_handlers() {
        let mut shs = GlobalStack::<String>::new();
        let mut shs1 = shs.clone();
        let mut shs2 = shs.clone();
        let mut shs3 = shs.clone();


        shs.push("Catarsis".to_owned());
        shs1.push("Catarsis 1".to_owned());
        shs2.push("Catarsis 2".to_owned());
        shs3.push("Catarsis 3".to_owned());


        // Check all the combinations
        let opposite = [&shs1, &shs1, &shs2, &shs3];
        let exhaust = [&shs1, &shs1, &shs2, &shs3];

        exhaust.into_iter().for_each(|v| {
            opposite.iter().for_each(|ov| {
                assert_eq!(v.reciprocate_clone(), ov.reciprocate_clone())
            });
        });
    }
}