use clap::{arg, Command};

fn main() {
    let matches = Command::new("devops")
        .version("1.0")
        .author("Andres F. <jandresfgz@gmail.com>")
        .about("Launch devops ticket in browser by providing its id")
        .arg(arg!(--id <VALUE>).required(true))
        .get_matches();

    let id = matches.get_one::<String>("id").expect("required");
    println!("id: {:?}", id);
}
