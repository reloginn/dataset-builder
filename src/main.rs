use app_args::parse_args;

mod app_args;

fn main() {
    let args = parse_args().expect("Cannot parse args");
    let input_path = args.input;
    let output_path = args.output;
    let mut iter = 0;
    let entries = std::fs::read_dir(input_path).expect("Cannot read directory");
    for entry in entries.flatten() {
        let path = entry.path();
        if let "webp" | "png" | "jpg" | "jpeg" = path.extension().unwrap().to_str().unwrap() {
            let file = std::fs::read(path.clone()).expect("Cannot read file");
            iter += 1;
            std::fs::write(
                output_path
                    .join(format!("{}", iter))
                    .with_extension(path.extension().expect("Cannot get extension")),
                file,
            )
            .expect("Cannot write an image");
            std::fs::File::create(output_path.join(format!("{}", iter)).with_extension("txt"))
                .expect("Cannot create file");
        }
    }
}
