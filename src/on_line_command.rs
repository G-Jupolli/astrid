use std::{
    fs,
    fs::File,
    io::{BufRead, BufReader, Write},
    path::Path,
};

pub fn on_line_command(
    file_path: String,
    prefix: String,
    suffix: String,
    no_new_line: bool,
    remove_original: bool,
    dir: Option<String>,
) {
    let f = match File::open(&file_path) {
        Ok(file) => file,
        Err(e) => {
            println!(
                "  - \x1b[91mFailed\x1b[0m: Failed to open file {}",
                &file_path
            );
            println!("{}", e.to_string());
            std::process::exit(1);
        }
    };

    let file_path = Path::new(&file_path);

    let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();

    let mut output_file_name = match file_name.split_once('.') {
        Some((a, b)) => format!("{a}_p.{b}"),
        None => format!("{file_name}_p.txt"),
    };

    output_file_name = match dir {
        Some(d) => format!("{d}/{output_file_name}"),
        None => {
            let mut ansestors = file_path.ancestors();
            let _ = ansestors.next();

            let x = ansestors.next().unwrap().to_str().unwrap().to_string();

            format!("{x}/{output_file_name}")
        }
    };

    let output_path = Path::new(&output_file_name);
    let mut output_file = File::create(&output_path).unwrap();

    let suffix = match no_new_line {
        true => suffix,
        false => format!("{suffix}\n"),
    };

    for line in BufReader::new(f).lines() {
        match line {
            Ok(data) => {
                let output = format!("{prefix}{data}{suffix}");

                if let Err(e) = output_file.write(output.as_bytes()) {
                    println!("  - \x1b[91mFailed\x1b[0m: Failed write to line");
                    println!("{}", e.to_string());
                    std::process::exit(1);
                };
            }
            Err(e) => {
                println!("  - \x1b[91mFailed\x1b[0m: Failed to parse line");
                println!("{}", e.to_string());
                std::process::exit(1);
            }
        }
    }

    if remove_original {
        if let Err(e) = fs::remove_file(file_path) {
            println!("  - \x1b[91mFailed\x1b[0m: Failed to remove original");
            println!("{}", e.to_string());
            std::process::exit(1);
        }
        println!("  - \x1b[92mRemoved\x1b[0m: {:?}", file_path);
    }

    println!("  - \x1b[92mComplete\x1b[0m: {output_file_name}");
}
