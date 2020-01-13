use std::io;

fn main() {
    println!("Temperature converter");
    loop {
        println!("\nExample input: 32 F");
        println!("Example output: Converted temp is 0° C");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.split_ascii_whitespace().collect::<Vec<&str>>();
        let temp: f64 = match input[0].trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let new_temp = match input[1].to_uppercase().as_str() {
            "C" => temp * 1.8 + 32.,
            "F" => (temp - 32.) / 1.8,
            _ => continue,
        };

        println!("Converted temp is {}° {}", new_temp, input[1]);
        break;
    }
}
