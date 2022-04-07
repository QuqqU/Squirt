use eval::Eval;
use object::Object;
use std::io::Write;

fn prompt(name: &str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error REP0001: Fail to read a line");

    return line.trim().to_string();
}

pub fn repl() {
    let eval = Eval::new();
    loop {
        match prompt("> ").as_str() {
            "exit" | "quit" => break,
            "info" => {
                println!(
                    "SPLI: Squirt Programming Language Interpreter\
                    \nSPLI is made by QuqqU.\
                    \nIf any problem, feel free to leave issue.\
                    \n\nSPLI github : https://github.com/QuqqU/Squirt"
                );
            }
            input => {
                let e = eval.run(input.to_owned());
                if !e.is_empty() && e != object::NULL.inspect() {
                    println!("{}", e);
                }
            }
        }
    }
}
