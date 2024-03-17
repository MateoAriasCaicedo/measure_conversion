use crate::input::{read_number, read_number_from_range};

#[derive(Clone, Copy)]
pub enum Conversion {
    FahrenheitToCelsius,
    CelsiusToFahrenheit,
}

pub enum Measurement {
    Fahrenheit,
    Celsius,
}

pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    return (celsius * 1.8) + f64::from(32);
}

pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    return (fahrenheit - f64::from(32)) * f64::from(5) / f64::from(9);
}

pub fn conversion_to_measurement(conversion: Conversion) -> Measurement {
    return match conversion {
        Conversion::FahrenheitToCelsius => Measurement::Celsius,
        Conversion::CelsiusToFahrenheit => Measurement::Fahrenheit,
    };
}

pub fn measurement_to_string(measurement: Measurement) -> String {
    return match measurement {
        Measurement::Fahrenheit => "F°".to_string(),
        Measurement::Celsius => "C°".to_string()
    };
}

const CONVERSION_OPTIONS: [f64; 2] = [1.0, 2.0];

pub fn read_conversion_option() -> Conversion {
    println!("1. From Fahrenheit to Celsius");
    println!("2. From Celsius to Fahrenheit");

    let conversion_option: f64 = read_number_from_range(&CONVERSION_OPTIONS);

    return if conversion_option == CONVERSION_OPTIONS[0] {
        Conversion::FahrenheitToCelsius
    } else {
        Conversion::CelsiusToFahrenheit
    };
}

pub fn read_conversion_value() -> f64 {
    println!("Enter the value to convert");
    return read_number();
}

pub fn calculate_conversion(conversion_value: f64, conversion_option: Conversion) -> f64 {
    return match conversion_option {
        Conversion::CelsiusToFahrenheit => celsius_to_fahrenheit(conversion_value),
        Conversion::FahrenheitToCelsius => fahrenheit_to_celsius(conversion_value),
    };
}

