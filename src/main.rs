mod template;

use std::env;
use std::process;

fn main()  {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: rainr <template> <file>");
        process::exit(1);
    }

    let template = &args[1];
    let file_name = &args[2];

    match template.as_str() {
        "basic" => template::gen_basic(file_name),
        "commands" => template::gen_commands(file_name),
        _ => {
            eprintln!("Invalid template type: {}", template);
            process::exit(1);
        }
    }
}
