use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long)]
   panic: bool,
}

fn main() {
    let args = Args::parse();
    println!("Hello, world!");

    if args.panic {
        always_panic();
    }
}

fn always_panic() {
    panic!("what? weird.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_always_panic() {
        always_panic();
    }
}