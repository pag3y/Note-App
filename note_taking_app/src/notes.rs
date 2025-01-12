use std::fs; // Import the fs module, which provides file system operations
use std::io::{self, Write}; // Import the Write trait, and the io module, which provides input/output functionality
use std::path::Path; // Import the Path struct, which represents file paths


// Function to add a new note
pub fn add_note(dir: &str, title: &str, content: &str) -> io::Result<()> { // The function takes a directory path, a title, and content as arguments, and returns an io::Result
    let file_path = format!("{}/{}.md", dir, title); // Create a file path by combining the directory path and the title
    let mut file = fs::File::create(file_path)?; // Create a new file at the specified path
    file.write_all(content.as_bytes())?; // Write the content to the file
    println!("Note '{}' added successfully!", title); // Print a success message
    Ok(()) // Return an Ok result
}

// Function to list all notes
pub fn list_notes(dir: &str) -> io::Result<()> { // The function takes a directory path as an argument, and returns an io::Result
    let entries = fs::read_dir(dir)?; // Read the contents of the directory

    println!("Notes in '{}':", dir); // Print a message with the directory name
    for entry in entries { // Iterate over the directory entries
        let entry = entry?; // Unwrap the entry
        if let Some(file_name) = entry.path().file_name() { // Get the file name of the entry
            if let Some(name) = file_name.to_str() { // Convert the file name to a string
                println!("- {}", name); // Print the file name
            }
        }
    }
    Ok(())
}

// Function to view a note's content
pub fn view_note(dir: &str, title: &str) -> io::Result<()> { // The function takes a directory path and a title as arguments, and returns an io::Result
    let file_path = format!("{}/{}.md", dir, title); // Create the file path
    if !Path::new(&file_path).exists() { // Check if the file exists
        return Err(io::Error::new( // Return an error if the file does not exist
            io::ErrorKind::NotFound, // Set the error kind to NotFound
            format!("Note '{}' does not exist", title), // Set the error message
        ));
    }

    let content = fs::read_to_string(file_path)?; // Read the content of the file
    println!("--- Content of '{}' ---", title); // Print a message with the note title
    println!("{}", content); // Print the content of the note
    println!("------------------------"); // Print a separator
    Ok(()) // Return an Ok result
}

pub fn delete_note(dir: &str, title: &str) -> io::Result<()> { // The function takes a directory path and a title as arguments, and returns an io::Result
    let file_path = format!("{}/{}.md", dir, title); // Create the file path
    if !Path::new(&file_path).exists() { // Check if the file exists
        return Err(io::Error::new( // Return an error if the file does not exist
            io::ErrorKind::NotFound, // Set the error kind to NotFound
            format!("Note '{}' does not exist", title), // Set the error message
        ));
    }

    fs::remove_file(file_path)?; // Remove the file
    println!("Note '{}' deleted successfully!", title); // Print a success message
    Ok(())
}

pub fn search_notes_by_title(dir: &str, query: &str) -> io::Result<()> { // The function takes a directory path and a query string as arguments, and returns an io::Result
    let entries = fs::read_dir(dir)?; // Read the contents of the directory

    println!("Searching for notes with '{}' in the title:", query); // Print a message with the search query
    let mut found = false; // Initialize a flag to track if any notes are found

    for entry in entries { // Iterate over the directory entries
        let entry = entry?; // Unwrap the entry
        if let Some(file_name) = entry.path().file_name() { // Get the file name of the entry
            if let Some(name) = file_name.to_str() { // Convert the file name to a string
                if name.contains(query) { // Check if the file name contains the search query
                    println!("- {}", name); // Print the file name
                    found = true; // Set the flag to true
                }
            }
        }
    }

    if !found { 
        println!("No notes found with '{}' in the title.", query); // Print a message if no notes are found
    }

    Ok(())

}

pub fn search_notes_by_content(dir: &str, query: &str) -> io::Result<()> { // The function takes a directory path and a query string as arguments, and returns an io::Result
    let entries = fs::read_dir(dir)?; // Read the contents of the directory

    println!("Searching for notes containing '{}' in the content:", query); // Print a message with the search query
    let mut found = false; // Initialize a flag to track if any notes are found

    for entry in entries { // Iterate over the directory entries
        let entry = entry?; // Unwrap the entry
        if let Some(file_name) = entry.path().file_name() { // Get the file name of the entry
            if let Some(name) = file_name.to_str() { // Convert the file name to a string
                let file_path = entry.path(); // Get the file path
                if let Ok(content) = fs::read_to_string(&file_path) { // Read the content of the file
                    if content.contains(query) { // Check if the content contains the search query
                        println!("- {} (contains match)", name); // Print the file name with a match indicator
                        found = true; // Set the flag to true
                    }
                }
            }
        }
    }

    if !found {
        println!("No notes found containing '{}' in the content.", query); // Print a message if no notes are found
    }

    Ok(())
}