use app_args::parse_args;

mod app_args;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let args = parse_args().expect("Cannot parse args");
    let input_path = args.input;
    let output_path = args.output;
    let mut iter = 0;
    std::fs::read_dir(input_path)?.flatten().for_each(|entry| {
        if let "png" | "jpg" | "jpeg" = entry.path().extension().unwrap().to_str().unwrap() {
            let file = std::fs::read(entry.path()).expect("Cannot read file");
            iter += 1;
            std::fs::write(
                output_path
                    .join(format!("{}", iter))
                    .with_extension(entry.path().extension().expect("Cannot get extension")),
                file,
            )
            .expect("Cannot write an image");
            std::fs::File::create(output_path.join(format!("{}", iter)).with_extension("txt"))
                .expect("Cannot create file");
        }
    });
    Ok(())
}
