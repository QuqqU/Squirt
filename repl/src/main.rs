use repl::repl;

fn main() {
    println!(
        "
     ___                         ___                       ___                 
    /  /\\          ___          /__/\\        ___          /  /\\          ___   
   /  /:/_        /  /\\         \\  \\:\\      /  /\\        /  /::\\        /  /\\  
  /  /:/ /\\      /  /::\\         \\  \\:\\    /  /:/       /  /:/\\:\\      /  /:/  
 /  /:/ /::\\    /  /:/\\:\\    ___  \\  \\:\\  /__/::\\      /  /:/~/:/     /  /:/   
/__/:/ /:/\\:\\  /  /:/~/::\\  /__/\\  \\__\\:\\ \\__\\/\\:\\__  /__/:/ /:/___  /  /::\\   
\\  \\:\\/:/~/:/ /__/:/ /:/\\:\\ \\  \\:\\ /  /:/    \\  \\:\\/\\ \\  \\:\\/:::::/ /__/:/\\:\\  
 \\  \\::/ /:/  \\  \\:\\/:/__\\/  \\  \\:\\  /:/      \\__\\::/  \\  \\::/~~~~  \\__\\/  \\:\\ 
  \\__\\/ /:/    \\  \\::/        \\  \\:\\/:/       /__/:/    \\  \\:\\           \\  \\:\\
    /__/:/      \\__\\/          \\  \\::/        \\__\\/      \\  \\:\\           \\__\\/
    \\__\\/                       \\__\\/                     \\__\\/                

Squirt Programming Language Interpreter
By QuqqU

- To get more info, \"info\"
- To quit, \"exit\" or \"quit\"

"
    );
    repl();
}

// let factorial = fn(n) { if(n == 0) { 1 } else { n * factorial(n - 1) } }; factorial(5);
// let add = fn(x, y) { x + y }; add(1, add(2, 3));
// let fibo = fn(n) { if(n < 2) { 1 } else { fibo(n - 1) + fibo(n - 2) } }; fibo(5);

// let closure = fn(x) { fn(y) { x + y } }
// let inner = closure(10)
// inner(5)
