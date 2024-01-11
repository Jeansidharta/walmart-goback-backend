use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// The port in which the server will open
    #[arg(short, long, default_value = "8001")]
    port: u16,

    /// Where to store the database file
    #[arg(short, long)]
    database_path: Option<PathBuf>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    walmart_goback_backend::server::server(args.port, args.database_path).await;
}
