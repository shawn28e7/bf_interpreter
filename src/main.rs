use bf_interpreter;
use std::{env, process};

fn main()
{
    let file_path: String = bf_interpreter::strip_args(env::args()).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    let code: String = bf_interpreter::read_code(&file_path).unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        process::exit(1);
    });

    let code: Vec<bf_interpreter::Status> = bf_interpreter::check_n_format_code(&code)
        .unwrap_or_else(|err| {
            eprintln!("Error: {err}");
            process::exit(1);
        });

    if let Err(e) = bf_interpreter::run(&code)
    {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
