//Assignment1: Temp Converter
const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celcius(f: f64) -> f64{
    (f - 32.0) * 5.0 / 9.0
}

fn celcius_to_fahrenheit(c: f64) -> f64{
    (c * 9.0 / 5.0) + 32.0
}

fn main() {

    let mut temp_f: f64 = FREEZING_POINT_F;
    
    let temp_c = fahrenheit_to_celcius(temp_f);
    println!("{}°F = {:.2}°C", temp_f, temp_c);

    for _ in 0..5{
        temp_f += 1.0;
        let temp_c = fahrenheit_to_celcius(temp_f);
        println!("{}°F = {:.2}°C", temp_f, temp_c);
    }
}