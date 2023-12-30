mod input;

fn main() {
    let filename = "day1.txt";
    let lines = input::get_lines(filename);
    let mut sum: u64 = 0;

    for s in lines{
        sum += find_calibration(&s);
    }

    println!("{sum}");
}

fn find_calibration(s: &String) -> u64 {
    let mut first = ' ';
    let mut last = ' ';

    // Find first and last digits
    for c in s.chars() {
        if c.is_digit(10) {
            if first == ' ' { first = c }
            last = c;
        }
    }

    // Concatenate and return calibration value
    let calibration_string = first.to_string() + &last.to_string();
    return calibration_string.parse::<u64>().expect("Not a digit");
}