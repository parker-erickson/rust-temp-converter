use std::io;

fn main() {
    let mut input_temp: String = String::new();
    let mut input_unit: String = String::new();

    io::stdin()
        .read_line(&mut input_temp)
        .expect("failed to read from stdin");

    io::stdin()
        .read_line(&mut input_unit)
        .expect("failed to read from stdin");

    let parsed_num: f32 = input_temp.trim().parse().expect("Expect a number");
    let trimmed_unit: String = input_unit.trim().to_uppercase();

    temp_converter(parsed_num, &trimmed_unit.to_owned());  
}

fn temp_converter(temp:f32, unit:&str) {
    match unit {
        "F" => println!("{}", f_to_c(temp)),
        "C" => println!("{}", c_to_f(temp)),
        _ => println!("error")
    }    
}

fn c_to_f(c:f32) -> f32 {
    let result: f32 = (c * 1.8) + 32.0;
    return result;
}

fn f_to_c(f:f32) -> f32 {
    return (f-32.0)*(0.56);
}
