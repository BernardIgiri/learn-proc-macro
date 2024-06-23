use learn_proc_macro::Reflective;

#[derive(Reflective)]
struct Rectangle {
    height: i32,
    width: i32,
}

fn main() {
    let square = Rectangle {
        height: 20,
        width: 200,
    };
    println!("struct type: {}", square.name());
    println!("field names: {:#?}", square.field_names());
}
