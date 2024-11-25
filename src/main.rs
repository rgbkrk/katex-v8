use katex_v8::{render, Error, Opts};

fn main() -> Result<(), Error> {
    let input = std::env::args().nth(1).unwrap_or_else(|| {
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");
        buffer.trim().to_string()
    });

    let opts = Opts::new().display_mode(true);
    let html = render(&input, &opts)?;
    println!("{}", html);
    Ok(())
}
