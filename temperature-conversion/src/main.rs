use std::io;

fn main() {

    let input_unit = loop {
        let mut input_unit = String::new();
        println!("Input unit (f/c)?");
        match io::stdin().read_line(&mut input_unit) {
            Ok(_) => 
                match input_unit.trim() {
                    "f" => break "f",
                    "c" => break "c",
                    _ => continue,
                },
            Err(_) => continue,
        }
    };


    println!("Input temperature.");


    let input_temperature_value: f64 = loop {
        let mut input_temperature_value = String::new();
        match io::stdin().read_line(&mut input_temperature_value) {
            Ok(_) => (),
            Err(_) => println!("Input a number."),
        }
        match input_temperature_value.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Input a number."),
        }
    };

    if input_unit == "f" {
        let result = (5.0 / 9.0) * (input_temperature_value - 32.0);
        println!("{result}C");
    } else {
        println!("{input_unit}");
        let result = (input_temperature_value * (5.0 / 9.0)) + 32.0;
        println!("{result}F");
    };
}
