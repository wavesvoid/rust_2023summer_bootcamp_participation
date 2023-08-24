use std::marker::PhantomData;
use std::cell::RefCell;
use rand::{thread_rng, Rng, rngs::ThreadRng};



const DEFAULT_FACT: &str = "Unfortunately there are no facts for this case";
const VEC_FACTS: [&str; 4] = [
    "Vec is stored on the heap",
    "Vec is a Smart Pointer",
    "Vec stores pointer to data, length and capacity",
    "Vec stores meta info on the stack whilst the data under the pointer is stored on the heap",
];
const STRING_FACTS: [&str; 4] = [
    "String is actually a Vec under the hood",
    "String is a smart pointer",
    "String is deep on the heap",
    "UTF-8 Strings are blazingly slow (comparing to ASCII)",
];


/// This trait is needed to be imported as well
/// to be able to get facts
pub trait GetFacts {
    fn fact(&self) -> &str {
        DEFAULT_FACT
    }
}

/// Represents a Fact, stores additional ThreadRng to avoid reinitialization
pub struct Fact<T>(PhantomData<T>, RefCell<ThreadRng>);

impl<Item> Fact<Item> {
    pub fn new() -> Self {
        Self (PhantomData, RefCell::new(thread_rng()))
    }

    /// Get a random number within the range from 0 to given MAX
    fn get_rand(&self, flen: usize) -> usize {
        self.1.borrow_mut().gen_range(0..flen)
    }

    /// Internally compute current fact from available facts
    /// given the target "name" which we need the fact for
    fn inner_fact(&self, name: &str) -> &'static str {
        let facts = match name {
            "vector" => VEC_FACTS.as_slice(),
            "string" => STRING_FACTS.as_slice(),
            _ => &[DEFAULT_FACT] // actually never will fall here
        };
        
        facts[self.get_rand(facts.len())]
    }
}


impl<T> GetFacts for Fact<Vec<T>> {
    fn fact(&self) -> &str {
        self.inner_fact("vector")
    }
}


impl GetFacts for Fact<String> {
    fn fact(&self) -> &str {
        self.inner_fact("string")
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::any::Any;


    #[test]
    fn test_facts() {
        let vec_fact: Fact<Vec<i32>> = Fact::new();
        let str_fact: Fact<String> = Fact::new();

        assert!(VEC_FACTS.contains(&vec_fact.fact()));
        assert!(STRING_FACTS.contains(&str_fact.fact()));
    }

    #[test]
    #[should_panic]
    fn test_missing_target() {
        let some_fact: Fact<i32> = Fact::new();
        
        check(Box::new(some_fact));

        fn check<T: Any>(t: T) {
            let value_any = &t as &dyn Any;
            match value_any.downcast_ref::<Box<dyn GetFacts>>() {
                Some(_) => {},
                None => panic!("The type does not implement the trai"),
            }
        }
    }
}