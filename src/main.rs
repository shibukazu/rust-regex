mod engine;

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn match_file(expr: &str, file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        // lineの途中に対してもマッチするように先頭をずらして試行する
        for (i, _) in line.chars().enumerate() {
            if engine::do_matching(expr, &line[i..])? {
                println!("{}", line);
                break;
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        eprintln!("Usage: {} <expr> <file>", args[0]);
        std::process::exit(1);
    } else {
        match_file(&args[1], &args[2])?;
    }

    Ok(())
}
