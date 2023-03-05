use colored::*;
use std::io::Write;
use std::path::{PathBuf, Path};
use std::fs::*;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    New {
        name: String,
    },
    Init {
        
    },
    Compile {
    
    },
    Run {
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::New { name }) => {
            let result = create_dir(name);
            match result {
                Ok(_) => {
                    create_dir(Path::new(name).join("src")).unwrap();
                    create_dir(Path::new(name).join("build")).unwrap();
                    create_dir(Path::new(name).join("local-packages")).unwrap();
                    let mut src_file = File::create(Path::new(name).join("src").join("main.skt")).unwrap();
                    src_file.write_all(b"println(\"Hello, world!\")\n");
                    println!("Successfully created project '{}' ", name.green());
                    println!("To get started, run {} {}", "cd".yellow(), name.yellow());
                    
                },
                Err(err) => {println!("Error: {}", err);}
            }
        },
        Some(Commands::Init {}) => {
            
        },
        Some(Commands::Compile {}) => {
            
        },
        Some(Commands::Run {}) => {
            
        },
        None => {}
    }

}