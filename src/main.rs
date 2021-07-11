use clap::{App, Arg};

mod errorsystem;
mod lang;

fn main() {
    let matches = App::new("RustGlass")
        .version("1.0.0")
        .author("Sullivan B")
        .about("Dynamically typed language written in RustLang")
        .arg(Arg::with_name("run")
            .index(1)
            .takes_value(true))
        .get_matches();

    if let Some(i) = matches.value_of("run") {
        println!("{}", i);
    }
}
