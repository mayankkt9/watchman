//! This example shows how to initiate a subscription and print out
//! file changes as they are reported
use std::path::PathBuf;
use structopt::StructOpt;
use watchman_client::prelude::*;

#[derive(Debug, StructOpt)]
#[structopt(about = "Subscribe to watchman and stream file changes for a path")]
struct Opt {
    #[structopt(default_value = ".")]
    path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(err) = run().await {
        // Print a prettier error than the default
        eprintln!("{}", err);
        std::process::exit(1);
    }
    Ok(())
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let client = Connector::new().connect().await?;
    let resolved = client
        .resolve_root(CanonicalPath::canonicalize(opt.path)?)
        .await?;

    let (mut sub, initial) = client
        .subscribe::<NameOnly>(&resolved, SubscribeRequest::default())
        .await?;

    println!("{} {:#?}", sub.name(), initial);
    loop {
        let item = sub.next().await?;
        println!("{:#?}", item);
    }
}
