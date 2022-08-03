use std::fs;

pub fn write(data: String) {
    fs::write("./db.txt", format!("{}\n", data)).expect("Unable to write file");
}

pub fn read() {
    return fs::read_to_string("./db.txt").expect("Unable to read file");
}