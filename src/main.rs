use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::Path;

// Statically stored license files
const LICENSES: &[(&str, &str)] = &[
    ("OSH-WARE", include_str!("./osh-ware/license.md")),
    ("CHOYXONA-WARE", include_str!("./choyxona-ware/license.md")),
];

/// License manager for Yuri Katsuki
#[derive(Debug, Parser)]
#[command(name = "raisensu")]
#[command(about = "License manager for Yuri Katsuki", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create license files in the current working directory
    #[command(arg_required_else_help = true)]
    Create {
        /// Show the type of license file to create
        license: String,
    },
    /// List all license files
    List,
    /// Delete existing license file
    Delete,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Create { license } => {
            // Check if the license exists and write index
            let mut found: usize = 0;
            
            for (index, (name, _)) in LICENSES.iter().enumerate() {
                if name.to_lowercase() == license.to_lowercase() {
                    found = index;
                    break;
                }
            }
            
            // If license not found, exit
            if found == 0 {
                println!("License not found");
                return;
            }
            
            // Write license file
            let license = LICENSES[found].1;
            let file_name = format!("LICENSE-{}", LICENSES[found].0);
            std::fs::write(file_name, license).unwrap();
            println!("License file created");
        }
        Commands::List => {
            println!("Available licenses:");

            // List all license files stored at LICENSES constant
            for (name, _) in LICENSES {
                println!("- {}", name);
            }
        }
        Commands::Delete => {
            let mut found: Vec<String> = Vec::new();

            // List all files in the current directory as Vec<PathBuf>
            let paths = std::fs::read_dir(Path::new("."))
                .unwrap()
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, std::io::Error>>()
                .unwrap();

            // Iterate over the paths, displaying each entry.
            for path in paths {
                if path.is_file() {
                    let file_name = path.file_name().unwrap().to_str().unwrap();
                    let re = regex::Regex::new(r"(?i)LICENSE").unwrap();
                    if re.is_match(file_name) {
                        found.push(file_name.to_string());
                    }
                }
            }

            // If no license file found, exit
            if found.len() == 0 {
                println!("No license file found");
                return;
            }

            // Delete all license files
            for file in found {
                println!("Deleting {}", file);
                std::fs::remove_file(file).unwrap();
            }

            println!("Done!")
        }
    }
}
