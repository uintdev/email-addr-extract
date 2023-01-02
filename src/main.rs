use regex::Regex;
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

fn args_validate(args: &Vec<String>) -> bool {
    let mut res = true;
    if args.len() != 3 {
        res = false;
        // Get file name from path
        let file_path = Path::new(&args[0]);
        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        // Determine correct separator to use
        let mut dir_separator = "/";
        if env::consts::OS == "windows" {
            dir_separator = "\\";
        }

        eprint!("\n");
        eprintln!(
            "Usage: .{}{} <input_file> <output_file>",
            dir_separator, file_name
        );
    }
    res
}

fn file_read(file: &str) -> io::Result<File> {
    let file_opener = fs::File::open(file)?;
    Ok(file_opener)
}

fn file_process(file_handle: &File) -> io::Result<Vec<String>> {
    let reader = io::BufReader::new(file_handle);

    // Use a regular expression to search for any email addresses per line
    // Pattern courtesy of https://emailregex.com/
    let email_regex = Regex::new(
        r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#
    ).unwrap();

    let mut email_addresses: Vec<String> = Vec::new();

    println!("Extracting email addresses from input file...");

    // Read the lines of the file
    for line in reader.lines() {
        let line = line?;
        // Go through the captures for the selected line
        for capture in email_regex.captures_iter(&line) {
            let email = capture[0].to_string();
            // Add to list
            email_addresses.push(email);
        }
    }

    // Clear duplicates
    email_addresses.dedup();

    println!("{} email addresses extracted", email_addresses.len());

    Ok(email_addresses)
}

fn file_write(file: &str, content: &Vec<String>) -> io::Result<()> {
    // Make the extracted list into a string but joint by new line, then write
    fs::write(file, content.join("\n"))?;

    println!(
        "Email addresses written to output file '{}' successfully",
        file
    );
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Get software version
    let version = env!("CARGO_PKG_VERSION");

    // Banner
    print!("\n");
    println!("--- Email Addr Extract v{} ---", version);

    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    if !args_validate(&args) {
        return Ok(());
    }

    let input_file = &args[1];
    let output_file = &args[2];

    let file = match file_read(&input_file) {
        Ok(file) => file,
        Err(err) => return Err(format!("Failed to open file: {}", err).into()),
    };

    // Display information about the file selection
    print!("\n");
    println!("Input file: {}", input_file);

    let metadata = match fs::metadata(input_file) {
        Ok(metadata) => metadata,
        Err(err) => return Err(format!("Failed to process file metadata: {}", err).into()),
    };

    println!("Input file size: {}", metadata.len());
    println!("Output file: {}", output_file);
    print!("\n");

    let email_addr_results = match file_process(&file) {
        Ok(email_addr_results) => email_addr_results,
        Err(err) => return Err(format!("Failed to process file contents: {}", err).into()),
    };

    println!("Writing results to file...");

    // Write the email addresses to a new file
    match file_write(output_file, &email_addr_results) {
        Ok(()) => (),
        Err(e) => return Err(format!("Error writing to file: {}", e).into()),
    }

    print!("\n");

    Ok(())
}
