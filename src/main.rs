use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn create_file_list(file_paths: &[String], list_file_path: &str) -> std::io::Result<()> {
    let mut file = File::create(list_file_path)?;
    for path in file_paths {
        writeln!(file, "file '{}'", path)?;
    }
    Ok(())
}

fn concatenate_videos(output_file: &str, list_file_path: &str) -> std::io::Result<()> {
    let status = Command::new("ffmpeg")
        .arg("-f")
        .arg("concat")
        .arg("-safe")
        .arg("0")
        .arg("-i")
        .arg(list_file_path)
        .arg("-c")
        .arg("copy")
        .arg(output_file)
        .status()?;
    
    if status.success() {
        println!("Successfully concatenated video files into {}", output_file);
    } else {
        eprintln!("Failed to concatenate video files.");
    }
    
    Ok(())
}

fn extract_numeric_id(filename: &str) -> u64 {
    // Find the first numeric part of the filename and parse it as a u64
    filename.split(|c: char| !c.is_digit(10))
        .filter_map(|s| s.parse::<u64>().ok())
        .next()
        .unwrap_or(0)
}

fn main() -> std::io::Result<()> {
    // Get the current working directory
    let cwd = env::current_dir()?;
    let videos_dir = cwd.join("videos");

    // Check if the "videos" subdirectory exists
    if !videos_dir.exists() || !videos_dir.is_dir() {
        eprintln!("No 'videos' directory found in the current working directory.");
        return Ok(());
    }

    // Get the parent directory's name (assuming "videos" is in the format /path/to/parent/videos)
    let parent_dir_name = cwd.file_name().unwrap().to_str().unwrap();

    // Create the output file path based on the parent directory name
    let output_file = videos_dir.join(format!("{}-concatenated.webm", parent_dir_name));
    
    // Collect all .webm files in the "videos" directory and sort them by the extracted numeric ID
    let mut file_paths: Vec<(u64, String)> = vec![];
    for entry in fs::read_dir(&videos_dir)? {
        let entry = entry?;
        let path = entry.path();
        let path_str = path.to_str().unwrap();

        // Skip the output file if it already exists
        if path_str != output_file.to_str().unwrap() && path.extension().and_then(|s| s.to_str()) == Some("webm") {
            let filename = path.file_name().unwrap().to_str().unwrap();
            let numeric_id = extract_numeric_id(filename);
            file_paths.push((numeric_id, path_str.to_string()));
        }
    }

    if file_paths.is_empty() {
        eprintln!("No .webm files found in the 'videos' directory.");
        return Ok(());
    }

    // Sort the files by their extracted numeric IDs
    file_paths.sort_by_key(|(id, _)| *id);

    // Extract just the file paths, now sorted
    let sorted_file_paths: Vec<String> = file_paths.into_iter().map(|(_, path)| path).collect();

    // Create a text file listing all .webm files to concatenate
    let list_file_path = videos_dir.join("file_list.txt");
    create_file_list(&sorted_file_paths, list_file_path.to_str().unwrap())?;

    // Concatenate the .webm files into one output file
    concatenate_videos(output_file.to_str().unwrap(), list_file_path.to_str().unwrap())?;

    // Clean up the list file
    fs::remove_file(list_file_path)?;

    Ok(())
}

