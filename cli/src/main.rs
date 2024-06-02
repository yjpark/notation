use std::env;
use std::path::Path;

use notation_dsl::prelude::parse_hand_shape6;
use notation_svg::prelude::*;

use notation_cli::util;

pub fn main() {
    let shape = parse_hand_shape6("Shape ( 0 0 0 2 3 2 )");

    let mut svg = String::new();
    write_svg_shape6(&mut svg, shape.unwrap()).unwrap();
    println!("svg: {:#?}", svg);

    let current_dir = env::current_dir().unwrap();
    let path = Path::new(&current_dir).join("temp.svg");

    util::write_file(&path, &svg);
}