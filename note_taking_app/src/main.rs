use std::fs; // Import the fs module, which provides file system operations
use std::io::{self, Write}; // Import the Write trait, and the io module, which provides input/output functionality
use std::path::Path; // Import the Path struct, which represents file paths

fn main() {
    // Ensure the "notes" directory exists.
    let notes_dir = "notes"; // Define the directory name
    fs::create_dir_all(notes_dir).expect("Failed to create notes directory"); // Create the directory if it doesn't exist, and ignore errors

    // Example: Add a note
    add_note(notes_dir, "example", "This is a sample note.").unwrap(); // Add a note with the title "example" and content "This is a sample note."

    // Example: List all notes
    list_notes(notes_dir).unwrap(); 

    // Example: View a note
    view_note(notes_dir, "example").unwrap_or_else(|e| { // View the note with the title "example", and handle any errors
        eprintln!("Error: {}", e); // Print an error message, if any
    });

    use std::io::{stdin, stdout,Write}; // Import the stdin, stdout, and Write traits from the io module
    let mut input= String::new(); 

    println!("Enter a note to view:");
    print!("> ");
    stdout().flush().unwrap();  // Flush the output buffer to display the prompt

    stdin().read_line(&mut input).unwrap(); // Read the user input from the standard input
    let note_title = input.trim(); // Remove leading and trailing whitespace from the input

    //Attempt to view the note
    view_note(&notes_dir, note_title).unwrap_or_else(|e| { // View the note with the specified title, and handle any errors
        eprintln!("Error: {}", e); // Print an error message, if any
    });

}

// Function to add a new note
fn add_note(dir: &str, title: &str, content: &str) -> io::Result<()> { // The function takes a directory path, a title, and content as arguments, and returns an io::Result
    let file_path = format!("{}/{}.md", dir, title); // Create a file path by combining the directory path and the title
    let mut file = fs::File::create(file_path)?; // Create a new file at the specified path
    file.write_all(content.as_bytes())?; // Write the content to the file
    println!("Note '{}' added successfully!", title); // Print a success message
    Ok(()) // Return an Ok result
}

// Function to list all notes
fn list_notes(dir: &str) -> io::Result<()> { // The function takes a directory path as an argument, and returns an io::Result
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
fn view_note(dir: &str, title: &str) -> io::Result<()> { // The function takes a directory path and a title as arguments, and returns an io::Result
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
