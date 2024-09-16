
# vid_cat

**vid_cat** is a Rust-based tool that concatenates multiple `.webm` video files in a directory into a single video using `ffmpeg`. The tool automatically sorts video files by numeric identifiers found in their filenames and combines them in order.

## Features

- **Automatic Sorting**: The tool extracts numeric IDs from filenames and sorts them before concatenation.
- **Video Concatenation**: Uses `ffmpeg` to concatenate multiple `.webm` files into a single output file.
- **Directory Scanning**: Scans the `videos` directory for `.webm` files to concatenate.

## Requirements

- **ffmpeg**: The program uses `ffmpeg` to handle video concatenation, so make sure it is installed on your system.
- **Rust toolchain**: The tool is written in Rust, so you'll need Rust installed to compile and run the program.

## Installation

### Step 1: Clone the Repository

```bash
git clone https://github.com/your-username/vid_cat.git
cd vid_cat
```

### Step 2: Build the Project

```bash
cargo build --release
```

## Usage

Run the program in the directory containing a `videos` subdirectory with `.webm` files.

```bash
./target/release/vid_cat
```

### Example

1. Ensure that your working directory has a `videos` subdirectory with `.webm` files. 
2. Run the command:

```bash
./target/release/vid_cat
```

This will:
1. Scan the `videos` directory for `.webm` files.
2. Sort the files based on the numeric ID in their filenames.
3. Concatenate the files into a single output file named `<parent_directory_name>-concatenated.webm`.

### File Sorting

- The tool looks for numeric values in the filenames and uses those values to determine the order of concatenation. For example, files like `video_001.webm`, `video_002.webm`, and `video_003.webm` will be concatenated in that order.

### Example Output

```bash
Successfully concatenated video files into parent_dir-concatenated.webm
```

## License

This project is licensed under the GNU GPLv3 License. See the [LICENSE](LICENSE) file for more details.
