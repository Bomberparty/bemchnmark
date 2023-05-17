use std::time::Instant as instant;
use std::time::Duration as duration;
use std::env;

fn fibbonacchi(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibbonacchi(n - 1) + fibbonacchi(n - 2)
    } 
}

fn benchmark(diff: i32) -> duration {
    let start_time = instant::now();

    fibbonacchi(diff);

    let end_time = instant::now();

    end_time-start_time
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <parameters> <difficulty>. Use `help` as first argument for a help display", args[0]);
        return;
    } else if args[1] == "help" {
        println!("Usage: {} <parameters> <difficulty>.", args[0]);
        println!("Parameters:");
        println!("\t-m\tmultiple repeated test (multiple input)");
        println!("Usage: {} -m <times to repeat> <difficulty>\n", args[0]);
        println!("\t-ma\tmultiple repeated test (single input of average. Increased operations number!)");
        println!("Usage: {} -ma <times to repeat> <difficulty>\n", args[0]);
        println!("\thelp\tdisplay this");
    }

    if (args[1] == "-ma" || args[1] == "-m") && args.len() < 4 {
        println!("Usage: {} {} <times to repeat> <difficulty>", args[0], args[1]);
    }else if args[1] == "-m" {
        let rep: i32 = match args[2].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Argument is NaN!");
                return;
            }
        };

        let diff: i32 = match args[3].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Argument is NaN!");
                return;
            }
        };

        for _ in 0..rep {
            let time = benchmark(diff);
            println!("The time is: {:?}", time);
        }
        
        return;

    }else if args[1] == "-ma" {
        let rep: u32 = match args[2].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Argument is NaN!");
                return;
            }
        };

        let diff: i32 = match args[3].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Argument is NaN!");
                return;
            }
        };

        let mut time: duration = Default::default();
        for _ in 0..rep {
            time = time + benchmark(diff); 
        }
        time /= rep;
        println!("The average time is: {:?}", time);

        return;
    }else {
        let diff: i32 = match args[1].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Argument is NaN!");
                return;
            }
        };

        let time = benchmark(diff);
        println!("The time is: {:?}", time);

    }
    
}
