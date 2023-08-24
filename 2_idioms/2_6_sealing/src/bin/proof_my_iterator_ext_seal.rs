use step_2_6::my_iterator_ext;

struct Sometype;

impl Iterator for Sometype {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(2)
    }
}

// impl my_iterator_ext::nine_tail::Seal for SomeType {} // private module
impl my_iterator_ext::MyIteratorExt for Sometype {}



fn main() {}