use std::any::TypeId;

#[derive(Debug)]
struct My;

impl std::fmt::Display for My {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "My")
    }
}

impl MyError for My {
    fn type_id(&self, _: step_2_6::my_error::nine_tails::Sealing) -> TypeId
    where
        Self: 'static,
    {
        TypeId::of::<Self>()
    }
}

fn main() {}