extern crate core;

mod conversion;
mod input;
mod output;

use crate::conversion::{calculate_conversion, Conversion, read_conversion_option, read_conversion_value};
use crate::output::{print_conversion_banner, print_conversion_result};

fn main() {
    print_conversion_banner();

    let conversion_option: Conversion = read_conversion_option();
    let conversion_value: f64 = read_conversion_value();
    let result: f64 = calculate_conversion(conversion_value, conversion_option);

    print_conversion_result(result, conversion_option);
}