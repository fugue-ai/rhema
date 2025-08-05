/*
 * Copyright 2025 Cory Parent
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use clap::{Parser, Subcommand};
use rhema_api::{Rhema, RhemaResult};

#[derive(Parser)]
#[command(name = "rhema-api")]
#[command(about = "Rhema Protocol API")]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Suppress output
    #[arg(short, long)]
    quiet: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Show API information
    Info,
}

fn main() -> RhemaResult<()> {
    let cli = Cli::parse();
    
    let rhema = Rhema::new()?;
    
    match &cli.command {
        Some(Commands::Info) => {
            println!("Rhema API v{}", env!("CARGO_PKG_VERSION"));
            println!("Repository: {}", rhema.repo_root().display());
            Ok(())
        }
        
        None => {
            println!("Rhema API - Use --help to see available commands");
            Ok(())
        }
    }
} 