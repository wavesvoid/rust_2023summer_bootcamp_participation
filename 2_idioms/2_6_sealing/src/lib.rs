
//! # Errors
//! * MyError
//! 
//! ```rust,compile_fail
//! use std::any::TypeId;
//! 
//! #[derive(Debug)]
//! struct My;
//! impl std::fmt::Display for My {
//!     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//!         write!(f, "My")
//!     }
//! }
//! impl MyError for My {
//!     fn type_id(&self, _: step_2_6::my_error::nine_tails::Sealing) -> TypeId
//!     where
//!         Self: 'static,
//!     {
//!         TypeId::of::<Self>()
//!     }
//! }
//! ```
//! * MyIteratorExt
//! 
//! ```rust,compile_fail
//! use step_2_6::my_iterator_ext::MyIteratorExt;
//! struct Sometype;
//! impl Iterator for Sometype {
//!     type Item = i32;
//!     fn next(&mut self) -> Option<Self::Item> {
//!         Some(2)
//!     }
//! }
//! impl MyIteratorExt for Sometype {}
//! ```


pub mod my_error;
pub mod my_iterator_ext;
pub use self::{my_error::MyError, my_iterator_ext::MyIteratorExt};



