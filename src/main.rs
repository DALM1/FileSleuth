use std::fs;
use std::io::{self, Read};
use std::path::Path;
use walkdir::{WalkDir, DirEntry};
use crossterm::{ExecutableCommand, terminal, cursor};

fn main() -> io::Result<()> {
    io::stdout().execute(terminal::Clear(terminal::ClearType::All))?;
    io::stdout().execute(cursor::MoveTo(0, 0))?;


    println!("888888 88 88     888888 .dP\"Y8 88     888888 88   88 888888 88  88 ");
    println!("88__   88 88     88__   `Ybo.\" 88     88__   88   88   88   88  88 ");
    println!("88\"\"   88 88  .o 88\"\"   o.`Y8b 88  .o 88\"\"   Y8   8P   88   888888 ");
    println!("88     88 88ood8 888888 8bodP' 88ood8 888888 `YbodP'   88   88  88 ");
    println!("FileSleuth v1.0");
    println!("By DALM1");


    let mut input = String::new();

    loop {
        println!("\nEnter a search term to find files and directories (or type 'exit' to quit):");
        input.clear();
        io::stdin().read_line(&mut input)?;

        let trimmed_input = input.trim();

        if trimmed_input.to_lowercase() == "exit" {
            break;
        }

        let search_results = search_entire_filesystem(trimmed_input)?;

        if !search_results.is_empty() {
            println!("\nSearch Results:");
            for (i, result) in search_results.iter().enumerate() {
                println!("{}: {}", i + 1, result.display());
            }

            println!("\nEnter the number of the file or directory to open, or 'back' to search again:");
            input.clear();
            io::stdin().read_line(&mut input)?;
            let selection = input.trim();

            if selection.to_lowercase() == "back" {
                continue;
            }

            if let Ok(index) = selection.parse::<usize>() {
                if index > 0 && index <= search_results.len() {
                    let selected_path = &search_results[index - 1];
                    if selected_path.is_dir() {
                        list_directory_contents(selected_path)?;
                    } else if selected_path.is_file() {
                        view_file_contents(selected_path)?;
                    }
                } else {
                    println!("Invalid selection. Please try again.");
                }
            } else {
                println!("Invalid input. Please enter a number.");
            }
        } else {
            println!("No files or directories found containing '{}'.", trimmed_input);
        }
    }

    Ok(())
}

fn search_entire_filesystem(search_term: &str) -> io::Result<Vec<std::path::PathBuf>> {
    let root_dir = Path::new("/");
    println!("Searching in the entire file system from root directory: {}", root_dir.display());
    let mut results = Vec::new();

    for entry in WalkDir::new(root_dir).max_depth(3).into_iter().filter_map(|e| e.ok()) {
        println!("Searching in: {}", entry.path().display());  // Real-time feedback
        if matches_search(&entry, search_term) {
            results.push(entry.path().to_path_buf());
        }
    }

    Ok(results)
}

fn matches_search(entry: &DirEntry, search_term: &str) -> bool {
    let file_name = entry.file_name().to_string_lossy().to_lowercase();
    file_name.contains(&search_term.to_lowercase())
}

fn list_directory_contents(path: &Path) -> io::Result<()> {
    println!("\nContents of directory '{}':", path.display());
    for entry in WalkDir::new(path) {
        let entry = entry?;
        println!("{}", entry.path().display());
    }
    Ok(())
}

fn view_file_contents(path: &Path) -> io::Result<()> {
    println!("\nContents of file '{}':", path.display());

    let mut file = fs::File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    match String::from_utf8(buffer) {
        Ok(contents) => {
            for (i, line) in contents.lines().enumerate() {
                println!("{:4}: {}", i + 1, line);
            }
        },
        Err(_) => {
            println!("Error: File is not valid UTF-8 and cannot be displayed.");
        }
    }

    Ok(())
}
