use super::file_position::FilePosition;

pub fn error(message: &String, file_pos: Option<&FilePosition>) {
    let mut err_string = String::from("Error: ");
    err_string += message;
    err_string += "\n";
    let fp: String;
    err_string += match file_pos {
        Some(_) => {
            fp = file_pos.unwrap().position();
            &fp
        },
        None => ""
    };
    println!("\x1b[31m{}\x1b[0m\n", err_string)
}