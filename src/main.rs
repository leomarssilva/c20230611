use clap::Parser;
use rand::{seq::SliceRandom, SeedableRng};
use rand_chacha;

// Other RNGs: https://rust-random.github.io/book/guide-rngs.html
type DefaultRng = rand_chacha::ChaCha8Rng;

#[derive(Parser, Debug)]
#[command(author, version, about)]
/// Struct with the args passed as options for the application.
struct Args {
    /// Smallest number to print.
    #[arg(long, default_value_t = 1)]
    start: usize,

    /// Biggest number to print.
    #[arg(long, default_value_t = 10)]
    stop: usize,

    /// Prints all numbers on the same line and separated by a space.
    #[arg(long)]
    same: bool,

    /// Uses a specific seed for random operations.
    #[arg(long)]
    seed: Option<u64>,
}

/// Returns a print function that can process one
/// entry by line or all in the same line.
fn build_print_f(one_by_line: bool) -> Box<dyn Fn((usize, &usize))> {
    if one_by_line {
        return Box::new(|(_, number)| println!("{number}"));
    }

    Box::new(|(i, number)| {
        if i > 0 {
            print!(" ")
        }
        print!("{number}")
    })
}

/// Returns a instance of `DefaultRng` initialized
/// with a optional seed number or the entropy from the OS.
fn get_rand_num_gen(seed: Option<u64>) -> DefaultRng {
    match seed {
        Some(num) => DefaultRng::seed_from_u64(num),
        None => DefaultRng::from_entropy(),
    }
}

fn get_random_list(start: usize, stop: usize, rng: &mut DefaultRng) -> Vec<usize> {
    let mut numbers: Vec<usize> = (start..stop + 1).collect();
    numbers.shuffle(rng);
    numbers
}

fn main() {
    let args = Args::parse();

    let mut rng = get_rand_num_gen(args.seed);

    let print_number = build_print_f(!args.same);

    let numbers = get_random_list(args.start, args.stop, &mut rng);

    numbers.iter().enumerate().for_each(print_number)
}
