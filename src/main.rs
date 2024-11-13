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

#[cfg(test)]
mod tests {
    use crate::engine::do_matching;

    #[test]
    fn matching() {
        // ケース: パースエラー
        assert!(do_matching("+b", "bbb").is_err());
        assert!(do_matching("*b", "bbb").is_err());
        assert!(do_matching("?b", "bbb").is_err());
        assert!(do_matching("|b", "bbb").is_err());

        // ケース: パース成功、マッチ成功
        assert!(do_matching("abc|def", "abc").unwrap());
        assert!(do_matching("abc|def", "def").unwrap());
        assert!(do_matching("(abc)*", "abcabc").unwrap());
        assert!(do_matching("(abc)*", "").unwrap());
        assert!(do_matching("(abc)?", "abc").unwrap());
        assert!(do_matching("(abc)?", "").unwrap());
        assert!(do_matching("(abc)+", "abcabc").unwrap());
        assert!(do_matching("(abc)+", "abc").unwrap());
        assert!(do_matching("a|b|c", "b").unwrap());

        // ケース: パース成功、マッチ失敗
        assert!(!do_matching("abc|def", "ghi").unwrap());
        assert!(!do_matching("a+", "").unwrap());
        assert!(!do_matching("a+", "b").unwrap());
        assert!(!do_matching("abc?", "acb").unwrap());
    }
}
