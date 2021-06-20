use structopt::StructOpt;
use std::path::PathBuf;
use crate::prototypes::HashInfo;
use std::fs::File;
use std::io::Read;

pub mod prototypes;
pub use prototypes::identify_hash;

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
        let res = prototypes::identify_hash(prototypes::PROTOTYPES.clone(), input).await;
        print_result(res);
    }
    if let Some(infile) = args.infile {
        let mut opened = File::open(infile)?;
        let mut buf = String::new();
        opened.read_to_string(&mut buf)?;
        let hashes = buf.lines().collect::<Vec<&str>>();
        let mut futs = Vec::new();
        for hash in hashes {
            futs.push(prototypes::identify_hash(prototypes::PROTOTYPES.clone(), hash.to_string()));
        }
        for fut in futs {
            let res = fut.await;
            print_result(res);
        }
    }
    Ok(())

}

fn print_result(modes: Vec<HashInfo>) {
    let mut hashtypes = String::new();
    for mode in modes {
        if !mode.extended {
            let formatted = format!("[+] {} ", mode.name);
            hashtypes.push_str(&formatted);
            if let Some(cat) = mode.hashcat {
                hashtypes.push_str(&format!("[Hashcat Mode: {}] ", cat));
            }
            if let Some(john) = mode.john {
                hashtypes.push_str(&format!("[JtR Format: {}]", &john));
            }
            hashtypes.push_str("\n");
        }
    }
    hashtypes.push_str("\n");
    println!("{}", hashtypes);
}
