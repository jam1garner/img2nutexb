use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    image: PathBuf,
    nutexb: PathBuf,

    #[structopt(short, long)]
    name: Option<String>,
}

fn main() {
    let args = Args::from_args();

    let image = image::open(args.image).unwrap();

    let mut file = File::create(args.nutexb).unwrap();

    nutexb::writer::write_nutexb_from_png(
        args.name.as_deref().unwrap_or("file"),
        image,
        &mut file
    ).unwrap();
}
