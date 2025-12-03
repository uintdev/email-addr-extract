use regex::Regex;
use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

fn args_validate(args: &Vec<String>) -> Result<(), ()> {
    if args.len() != 3 {
        eprintln!();

        // Get file name from path
        let file_path: &Path = Path::new(&args[0]);
        let file_name: &OsStr = file_path.file_name().expect("Failed to gather file name");

        // Determine correct separator to use
        let dir_separator: &str = std::path::MAIN_SEPARATOR_STR;

        eprintln!(
            "Usage: .{dir_separator}{} <input_file> <output_file>",
            &file_name.to_string_lossy()
        );

        println!();

        return Err(());
    }

    Ok(())
}

fn file_read(file: &str) -> io::Result<File> {
    let file_opener: File = fs::File::open(file)?;
    Ok(file_opener)
}

fn file_process(file_handle: &File) -> io::Result<Vec<String>> {
    let reader: io::BufReader<&File> = io::BufReader::new(file_handle);

    // Use a regular expression to search for any email addresses per line
    let email_regex = Regex::new(
        r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#
    ).expect("Malformed regex pattern");

    let mut email_addresses: Vec<String> = Vec::new();

    println!("Extracting email addresses from input file...");

    // Read the lines of the file
    for line in reader.lines() {
        let line: String = line?;
        // Go through the captures for the selected line
        for capture in email_regex.captures_iter(&line) {
            let email: String = capture[0].to_string();
            // Add to list
            email_addresses.push(email);
        }
    }

    // Clear duplicates
    email_addresses.dedup();

    println!("{} email addresses extracted", email_addresses.len());

    Ok(email_addresses)
}

fn file_write(file: &str, content: &[String]) -> io::Result<()> {
    // Make the extracted list into a string but joint by new line, then write
    fs::write(file, content.join("\n"))?;

    println!("Email addresses written to output file '{file}' successfully");
    Ok(())
}

fn banner() {
    // Get software version
    let version: &'static str = env!("CARGO_PKG_VERSION");

    // Banner
    println!();
    println!(r"╔═══════════════════════════════════════════════════════════════╗");
    println!(r"║      _____                 _ _      _       _     _           ║");
    println!(r"║     | ____|_ __ ___   __ _(_) |    / \   __| | __| |_ __      ║");
    println!(r"║     |  _| | '_ ` _ \ / _` | | |   / _ \ / _` |/ _` | '__|     ║");
    println!(r"║     | |___| | | | | | (_| | | |  / ___ \ (_| | (_| | |        ║");
    println!(r"║     |_____|_| |_| |_|\__,_|_|_| /_/   \_\__,_|\__,_|_|        ║");
    println!(r"║                                                               ║");
    println!(r"║               _____      _                  _                 ║");
    println!(r"║              | ____|_  _| |_ _ __ __ _  ___| |_               ║");
    println!(r"║              |  _| \ \/ / __| '__/ _` |/ __| __|              ║");
    println!(r"║              | |___ >  <| |_| | | (_| | (__| |_               ║");
    println!(r"║              |_____/_/\_\\__|_|  \__,_|\___|\__|              ║");
    println!(r"║                                                               ║");
    println!(r"║                            v{version}                             ║");
    println!(r"║                                                               ║");
    println!(r"╚═══════════════════════════════════════════════════════════════╝");
}

fn main() -> Result<(), Box<dyn Error>> {
    // Banner
    banner();

    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    if args_validate(&args).is_err() {
        return Ok(());
    }

    let input_file: &String = &args[1];
    let output_file: &String = &args[2];

    let file: File = match file_read(input_file) {
        Ok(file) => file,
        Err(err) => return Err(format!("Failed to open file: {err}").into()),
    };

    // Display information about the file selection
    println!();
    println!("Input file: {input_file}");

    let metadata: fs::Metadata = match fs::metadata(input_file) {
        Ok(metadata) => metadata,
        Err(err) => return Err(format!("Failed to process file metadata: {err}").into()),
    };

    println!("Input file size: {}", metadata.len());
    println!("Output file: {output_file}");
    println!();

    let email_addr_results: Vec<String> = match file_process(&file) {
        Ok(email_addr_results) => email_addr_results,
        Err(err) => return Err(format!("Failed to process file contents: {err}").into()),
    };

    println!("Writing results to file...");

    // Write the email addresses to a new file
    match file_write(output_file, &email_addr_results) {
        Ok(()) => (),
        Err(err) => return Err(format!("Error writing to file: {err}").into()),
    }

    println!();

    Ok(())
}
