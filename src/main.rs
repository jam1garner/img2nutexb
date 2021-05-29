use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

use nutexb::writer::ToNutexb;

#[derive(StructOpt)]
struct Args {
    image: PathBuf,
    nutexb: PathBuf,

    #[structopt(short, long)]
    dds: bool,

    #[structopt(short, long)]
    unswizzled: bool,

    #[structopt(short, long)]
    name: Option<String>,
}

fn write<T: ToNutexb>(image: &T, file: &mut File, name: &str, unswizzled: bool) {
    if unswizzled {
        nutexb::writer::write_nutexb_unswizzled(
            name,
            image,
            file
        ).unwrap();
    } else {
        nutexb::writer::write_nutexb(
            name,
            image,
            file
        ).unwrap();
    }
}

fn main() {
    let args = Args::from_args();

    let mut file = File::create(args.nutexb).unwrap();
    let name = args.name.as_deref().unwrap_or("img2nutexb_file");
    if args.dds {
        let mut img_file = std::io::BufReader::new(File::open(args.image).unwrap());
        write(
            &ddsfile::Dds::read(&mut img_file).expect("Failed to read the dds file"),
            &mut file,
            name,
            args.unswizzled
        );
    } else {
        write(
            &image::open(args.image).expect("Failed to read the image file"),
            &mut file,
            name,
            args.unswizzled
        );
    }
}
