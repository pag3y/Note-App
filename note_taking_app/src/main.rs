mod notes; // Import the notes module from the notes.rs file

use std::fs; // Import the fs module, which provides file system operations

use notes::{add_note, list_notes, view_note, delete_note}; // Import the add_note, list_notes, view_note, and delete_note functions from the notes module

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

      // Prompt user for a note to delete
      println!("Enter the title of the note to delete:"); // Print a message to prompt the user
      print!("> "); // Print a prompt symbol
      stdout().flush().unwrap(); // Flush the output buffer to display the prompt
      input.clear(); // Clear the input buffer
      stdin().read_line(&mut input).unwrap(); // Read the user input from the standard input
      let note_title = input.trim(); // Remove leading and trailing whitespace from the input
      delete_note(notes_dir, note_title).unwrap_or_else(|e| { // Delete the note with the specified title, and handle any errors
          eprintln!("Error: {}", e); // Print an error message, if any
      });

}
