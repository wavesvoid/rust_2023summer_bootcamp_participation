use std::ops::{Deref, DerefMut};
use std::cell::RefCell;
use rand::prelude::*;



// use std::borrow::Borrow;
#[derive(Debug)]
struct Random<T>([T; 3], RefCell<ThreadRng>);


// Just for convenience in tests
impl<T: PartialEq> PartialEq for Random<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().zip(other.0.iter()).all(|tup| tup.0 == tup.1)
    }
}


impl<T> Random<T> {
    pub fn new(v: [T; 3]) -> Self {
        Self(v, RefCell::new(thread_rng()))
    }

    fn get_random(&self) -> usize {
        self.1.borrow_mut().gen_range(1..3) as usize
    }
}


impl<T> Deref for Random<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0[self.get_random()]
    }
}

impl<T> DerefMut for Random<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0[self.get_random()]
    }
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_random() {
        let data = [1,2,3];
        let rnd_1 = Random::new(data);
        let mut rnd_2 = Random::new(data);

        let random_num: &i32 = &rnd_1;
        let random_num_mut: &mut i32 = &mut rnd_2;


        assert!(data.iter().any(|v| v == random_num));
        assert!(data.iter().any(|v| v == random_num_mut));

        *random_num_mut = 9;

        assert!(&9 == random_num_mut);
        assert!(&mut 9 == random_num_mut);
    }
}