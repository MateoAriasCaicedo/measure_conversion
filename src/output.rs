use crate::conversion::{Conversion, conversion_to_measurement, measurement_to_string};

pub fn print_conversion_result(result: f64, conversion: Conversion) {
    print!(
        "The result of the conversion was: {result}{}",
        measurement_to_string(conversion_to_measurement(conversion))
    );
}

pub fn print_conversion_banner() {
    println!("Metric conversion application!");
}