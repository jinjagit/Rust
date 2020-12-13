use linecount::count_lines;
use rev_lines::RevLines;
use std::fs::File;
use std::io::BufReader;
use std::{thread, time, env, process};
use colored::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    // let args_n: usize = args.iter().count();

    check_args(&args);

    let filename: &str = &args[1];
    let phrase: &str = &args[2];
    let mut lines: usize = linecount(filename);

    loop {
        let line_count: usize = linecount(filename);

        if line_count > lines {
            let num_newlines: usize = line_count - lines;
            let newlines: Vec<String> = get_newlines(num_newlines, filename);

            // print lines from newline vec in reverse order == order in original file
            for i in 0..num_newlines {
                let raw_line = &newlines[num_newlines - i - 1];
                if raw_line.contains(phrase) {
                    let line = raw_line.replace(phrase, &("*#~".to_owned() + phrase + "*#~"));
                    let split: Vec<&str> = line.split("*#~").collect();

                    for p in split {
                        if p == phrase {
                            print!("{}", p.bright_blue().bold());
                        } else {
                            print!("{}", p);
                        }     
                    }
                    
                    print!("\n");
                }   
            }

            lines = line_count;
        }

        thread::sleep(time::Duration::from_secs(1));
    }
}

fn linecount(filename: &str) -> usize {
    count_lines(File::open(filename).unwrap()).unwrap()
}

fn get_newlines(num_newlines: usize, filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let rev_lines = RevLines::new(BufReader::new(file)).unwrap();
    let mut count: usize = 0;
    let mut newlines: Vec<String> = vec![];

    // add n last lines of file to newlines vec, starting from last line of file
    for line in rev_lines {
        newlines.push(line.clone());
        count += 1;

        if count == num_newlines {
            break;
        }
    }

    newlines
}

fn check_args(args: &Vec<String>) {
    if args.iter().count() == 1 {
        print!("{}", "Error! No arguments found.".red());
        print!(" Type 'tailit help' for help on usage.\n");
        process::exit(1);
    } else {
        if args[1] == "help" {
            print_help();
            process::exit(1);
        }
    }
}

fn print_help() {
    println!("Help - How to use tailit:\n");
    println!("16 basic colors:");
    print!("  1. {}", "black".black());
    print!("          2. {}", "red".red());
    print!("             3. {}", "green".green());
    println!("         4. {}", "yellow".yellow());
    print!("  5. {}", "blue".blue());
    print!("           6. {}", "magenta".magenta());
    print!("         7. {}", "cyan".cyan());
    println!("          8. {}", "white".white());
    print!("  9. {}", "bright_black".bright_black());
    print!("  10. {}", "bright_red".bright_red());
    print!("     11. {}", "bright_green".bright_green());
    println!("  12. {}", "bright_yellow".bright_yellow());
    print!(" 13. {}", "bright_blue".bright_blue());
    print!("   14. {}", "bright_magenta".bright_magenta());
    print!(" 15. {}", "bright_cyan".bright_cyan());
    println!("   16. {}", "bright_white".bright_white());
}

// Todo:

// Get word from args
// Get words from args
// Get words from 'example|another' type args
// Get options from args (in form -example)
// Options:
//   color (a number) - default is bright cyan, use color value 0 for default terminal text color.
//   regular (default is bold)
//   lines before (if exist)
//   lines after (if exist)
// Add details, examples and limitations to help
// ? Use codegen to generate code to style word(s) from option(s) args?
//   https://docs.rs/codegen/0.1.3/codegen/