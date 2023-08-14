use clap::{App, Arg};

fn main() {
    let matches = App::new("RPN Program")
        .version("1.0.0")
        .author("Your name")
        .about("Super awesome sample RPN calculator")
        .arg(Arg::with_name("formula_file")
            .value_name("FILE")
            .index(1)
            .required(false),)
        .arg(Arg::with_name("verbose")
            .short('v')
            .long("verbose")
            .multiple(false),)
        .get_matches(); 

    match matches.value_of("formula_file") {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified."),
    }
        
    let vetbose = matches.is_present("verbose");
    println!("Is verbosity specified?: {}", vetbose);
}

