use learn_proc_macro::{MetaData, Reflective};

#[derive(MetaData)]
#[metadata(author = "Eray Edin", serial_version = 4)]
#[derive(Reflective)]
struct Rectangle {
    #[metadata(author = "Brandon Fraisure")]
    height: i32,
    #[metadata(author = "Brian Fox")]
    width: i32,
}

fn main() {
    let square = Rectangle {
        height: 20,
        width: 200,
    };
    println!("struct type: {}", square.name());
    println!("field names: {:#?}", square.field_names());
    println!("author: {:#?}", square.author());
    println!("serial_version: {:#?}", square.serial_version());
    println!("field_authors: {:#?}", square.field_authors());
}
