use std::str::FromStr;

pub fn main(args: &[String]) {
    for loops in args {
        let range = match i32::from_str(loops) {
            Ok(end) => 0..=end,
            _ => 0..=0
        };

        for i in range {
            println!("{:}", i);
        }
    }
}
