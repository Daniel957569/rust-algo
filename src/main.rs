pub fn get_size(string: &String) -> i32 {
    string.len() as i32
}

pub fn get_digit(ch: char) -> i32 {
    match ch {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => -1,
    }
}

pub fn get_string_as_int(string: &String) -> i32 {
    let mut size = get_size(string);
    let mut result = 0;

    for ch in string.chars().into_iter() {
        result = get_digit(ch) * size;
        size -= 1;
    }

    println!("{}", result);
    return result;
}

pub fn multiply(num1: String, num2: String) -> String {
    get_string_as_int(&num1);
    num1
}

fn main() {
    multiply("22".to_string(), "33".to_string());
}
