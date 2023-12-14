use clap::Parser;

#[derive(Debug, Parser)]
pub struct Comms {
    /// Add an directory to dapu
    #[clap(short, long)]
    pub add: Option<String>,
        
    /// Remove an directory to dapu
    #[clap(short, long)]
    pub remove: Option<String>,

    /// Output the path of the selected directory
    #[clap(short, long)]
    pub only_path: bool
}
