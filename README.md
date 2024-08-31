# FileSleuth

FileSleuth is a command-line tool written in Rust that allows users to search through their file system for specific files or directories by name or extension. It provides an interactive interface to browse the results and view the contents of files or explore directories.

## Features

- **Search Across the Entire File System**: FileSleuth searches the entire file system starting from the root directory, providing comprehensive search results.
- **Interactive Result Browsing**: Users can select a file or directory from the search results to view its contents or list its contents.
- **Real-Time Search Feedback**: The tool displays the directories being searched in real-time, keeping users informed about the progress.
- **UTF-8 Handling**: FileSleuth gracefully handles files that are not in UTF-8 format, notifying the user if the file cannot be displayed.

## Installation

### Prerequisites

- Rust programming language installed. You can install Rust using [rustup](https://rustup.rs/).

### Building the Project

1. Clone the repository:

   ```bash
   git clone https://github.com/DALM1/FileSleuth.git
   cd FileSleuth


   cargo build --release

   cargo run
