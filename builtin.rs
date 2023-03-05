struct skatetype_String {
    data: String,
}

impl skatetype_String {
    fn new(data: String) -> Self {
        skatetype_String {
            data,
        }
    }
}

fn skatefunction_println(s:  skatetype_String) {
    println!("{}", s.data);
}
fn skatefunction_print(s:  skatetype_String) {
    print!("{}", s.data);
}
