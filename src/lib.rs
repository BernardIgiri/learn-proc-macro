use std::collections::HashMap;

pub use reflective_derive::{Reflective, MetaData};

pub trait Reflective {
    fn name(&self) -> &'static str;
    fn field_names(&self) -> Vec<&'static str>;
}

pub trait MetaData {
	fn author(&self) -> &'static str;
    fn serial_version(&self) -> usize;
    fn field_authors(&self) -> HashMap<&'static str, &'static str>;
}