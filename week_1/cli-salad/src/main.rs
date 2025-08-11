use clap::Parser;
use cli_salad::create_fruit_salad;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Paul P.",
    about = "CLI Fruit Salad Generator"
)]
struct Opts {
    #[clap(short, long)]
    number: usize,
}

fn main() {
    let opts = Opts::parse();

    let num_fruits = opts.number;

    // Store the result of create_fruit_salad
    let fruit_salad = create_fruit_salad(num_fruits);

    // Use the stored result in the println! macro
    println!("Fruit Salad created with {} fruits: {:?}", num_fruits, fruit_salad);
}