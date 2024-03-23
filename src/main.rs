use clap::{App, Arg};
fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("yamada")
        .about("test@ex.com")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input Text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_new_line")
                .short("n")
                .help("Don't print new line")
                .takes_value(false),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap().join(" ");
    let omit_new_line = matches.is_present("omit_new_line");

    println!("{}{}", text, if omit_new_line { "" } else { "\n" });
}
