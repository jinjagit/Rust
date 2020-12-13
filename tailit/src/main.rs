use colored::*;
use linecount::count_lines;
use rev_lines::RevLines;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::{env, process, thread, time};

fn main() {
    let args: Vec<String> = env::args().collect();
    let (filename, phrases): (&str, Vec<(&str, &str)>) = parse_args(&args);
    let mut lines: usize = 0;

    confirm_delete(filename);

    loop {
        let line_count: usize = linecount(filename);

        if line_count > 20000 {
            delete_file_contents(filename);
            print_deletion_notice(filename);
            lines = 0;
        } else if line_count > lines {
            run_search(filename, lines, line_count, &phrases);
            lines = line_count;
        }

        thread::sleep(time::Duration::from_secs(1));
    }
}

fn parse_args(args: &Vec<String>) -> (&str, Vec<(&str, &str)>) {
    if args[1] == "help" {
        print_help();
        process::exit(1);
    } else if args.iter().count() == 1 {
        print!("{}", "Error! No arguments found.".red());
        print!(" Type 'tailit help' for help on usage.\n");
        process::exit(1);
    } else if args.iter().count() == 2 {
        print!("{}", "Error! Insufficient arguments found.".red());
        print!(" Type 'tailit help' for help on usage.\n");
        process::exit(1);
    }

    let num_args = args.iter().count();
    let filename: &str = &args[1];
    let mut phrases: Vec<(&str, &str)> = vec![];

    for i in 2..num_args {
        if i % 2 == 0 {
            phrases.push((&args[i], &args[i + 1]));
        }
    }

    // println!("{:?}", phrases); // DEBUG

    (filename, phrases)
}

fn run_search(filename: &str, lines: usize, line_count: usize, phrases: &Vec<(&str, &str)>) {
    let num_newlines: usize = line_count - lines;
    let newlines: Vec<String> = get_newlines(num_newlines, filename);

    // print lines from newline vec in reverse order == order in original file
    for i in 0..num_newlines {
        let raw_line = &newlines[num_newlines - i - 1];

        // TODO: Get search-phrase for starting divider from option arg (default = nil)
        if raw_line.contains("Started") {
            print!(
                "\n{}{}{}\n\n",
                "---------------------------------- ".bright_yellow(),
                filename.bright_yellow(),
                " ----------------------------------".bright_yellow()
            );
        }

        for i in 0..phrases.iter().count() {
            let (phrase, color) = phrases[i];
            if raw_line.contains(phrase) {
                let line = raw_line.replace(phrase, &("*#~".to_owned() + phrase + "*#~"));
                let split: Vec<&str> = line.split("*#~").collect();

                for p in split {
                    if p == phrase {
                        print_highlighted_phrase(phrase, color);
                    } else {
                        print!("{}", p);
                    }
                }

                print!("\n");
            }
        }
    }
}

fn linecount(filename: &str) -> usize {
    count_lines(File::open(filename).unwrap()).unwrap()
}

// Return vec of newlines
// TODO: Return some extra preceeding lines too, to enable printing of any preceeding lines
// specified by user (since we count line numbers every second to detect changes, but process being
// logged may last longer than the period considered; the last second or less - e.g server logs for
// a page load)
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

fn confirm_delete(filename: &str) {
    println!(
        "{}{}{}",
        "Warning! About to delete contents of ".bright_yellow(),
        filename.bright_blue(),
        ". Continue? (y/n)".bright_yellow()
    );

    let input = get_input();

    if input == "y" || input == "Y" {
        delete_file_contents(filename);
        println!(
            "{}{}{}",
            "Deleted file contents. Now watching file ",
            filename.bright_blue(),
            "..."
        );
    } else {
        process::exit(1);
    }
}

fn delete_file_contents(filename: &str) {
    let result = delete_contents(filename);
    match result {
        Ok(_) => (),
        Err(e) => println!("error deleting file contents: {:?}", e),
    }
}

fn delete_contents(filename: &str) -> std::io::Result<()> {
    let f = File::create(filename)?;
    f.set_len(0)?;
    Ok(())
}

fn get_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {}
        Err(_no_updates_is_fine) => {}
    }
    input.trim().to_string()
}

fn print_deletion_notice(filename: &str) {
    println!(
        "\n\n{}",
        "========================================================="
            .bright_red()
            .bold()
    );
    println!(
        "{}{}",
        "  Warning! Deleted contents of ".bright_red(),
        filename.bright_blue()
    );
    println!(
        "{}",
        "  Re-run last operation to ensure tail log is complete."
    );
    println!(
        "{}\n\n",
        "========================================================="
            .bright_red()
            .bold()
    );
}

fn print_help() {
    println!("Help - How to use tailit:\n");
    println!("Note: Bright and non-bright colors may look identical on some terminals.\n");
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

fn print_highlighted_phrase(phrase: &str, color: &str) {
    match color {
        "0" => print!("{}", phrase.normal()),
        "1" => print!("{}", phrase.black()),
        "2" => print!("{}", phrase.red()),
        "3" => print!("{}", phrase.green()),
        "4" => print!("{}", phrase.yellow()),
        "5" => print!("{}", phrase.blue()),
        "6" => print!("{}", phrase.magenta()),
        "7" => print!("{}", phrase.cyan()),
        "8" => print!("{}", phrase.white()),
        "9" => print!("{}", phrase.bright_black()),
        "10" => print!("{}", phrase.bright_red()),
        "11" => print!("{}", phrase.bright_green()),
        "12" => print!("{}", phrase.bright_yellow()),
        "13" => print!("{}", phrase.bright_blue()),
        "14" => print!("{}", phrase.bright_magenta()),
        "15" => print!("{}", phrase.bright_cyan()),
        "16" => print!("{}", phrase.bright_white()),
        "0b" => print!("{}", phrase.normal().bold()),
        "1b" => print!("{}", phrase.black().bold()),
        "2b" => print!("{}", phrase.red().bold()),
        "3b" => print!("{}", phrase.green().bold()),
        "4b" => print!("{}", phrase.yellow().bold()),
        "5b" => print!("{}", phrase.blue().bold()),
        "6b" => print!("{}", phrase.magenta().bold()),
        "7b" => print!("{}", phrase.cyan().bold()),
        "8b" => print!("{}", phrase.white().bold()),
        "9b" => print!("{}", phrase.bright_black().bold()),
        "10b" => print!("{}", phrase.bright_red().bold()),
        "11b" => print!("{}", phrase.bright_green().bold()),
        "12b" => print!("{}", phrase.bright_yellow().bold()),
        "13b" => print!("{}", phrase.bright_blue().bold()),
        "14b" => print!("{}", phrase.bright_magenta().bold()),
        "15b" => print!("{}", phrase.bright_cyan().bold()),
        "16b" => print!("{}", phrase.bright_white().bold()),
        _ => print!("{}", phrase.normal()),
    }
}

// Todo:

// Add thorough arguments syntax checking, etc.
// Get multiple search phrases from 'example|another' type args
//     -> issue: single quotes not included in &str from args
// Get options from args (in form -example)
// Options:
//   color (a number) - default is bright cyan, use color value 0 for default terminal text color.
//   regular (default is bold)
//   lines before (if exist)
//   lines after (if exist)
// Add details, examples and limitations to help
// Get search-phrase for starting divider from option arg (default = nil)

// Example usage:

// $ tailit development.log Started 11b Completed 11b Rendered 15b Parameters 14b ResultsController 13b
