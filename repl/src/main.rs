use lexer::Lexer;
use parser::Parser;
use std::io::Write;
use std::time::SystemTime;

fn prompt(name: &str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: Could not read a line");

    return line.trim().to_string();
}

fn main() {
    loop {
        let input = prompt("> ");
        if input == "now" {
            let unixtime = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap();
            println!("Current Unix time is {:?}", unixtime);
        }
        else if input == "exit" {
            break;
        }
        else {
            let program = Parser::new(Lexer::new(input)).parse_program();
            if !program.statements.is_empty() {
                println!("{}", program.statements[0].to_string());
                let e = eval::eval(&program.statements[0]);
                println!("value : {}", e.inspect());
            }
        }
    }
}
