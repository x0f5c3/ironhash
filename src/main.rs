use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;

pub use ironhash::HashResult;
#[derive(StructOpt, Debug)]
struct App {
    #[structopt(conflicts_with("infile"))]
    input: Option<String>,
    #[structopt(short, long)]
    infile: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args = App::from_args();
    if let Some(input) = args.input {
        let res = HashResult::new(input).await;
        println!("{}", res);
    }
    if let Some(infile) = args.infile {
        let mut opened = File::open(infile)?;
        let mut buf = String::new();
        opened.read_to_string(&mut buf)?;
        let hashes = buf.lines().collect::<Vec<&str>>();
        let mut futs = Vec::new();
        for hash in hashes {
            futs.push(HashResult::new(hash.to_string()));
        }
        for fut in futs {
            let res = fut.await;
            println!("{}", res);
        }
    }
    Ok(())
}
