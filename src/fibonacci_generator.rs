use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct FibonacciGenerator {
    #[arg(short, long, default_value_t = 0)]
    from: i128,
    #[arg(short, long, default_value_t = 1)]
    to: i128,
}

impl FibonacciGenerator {
    pub fn generate(&self) {
        let mut a = 0;
        let mut b = 1;
        loop {
            let tmp = a;
            a = a + b;
            b = tmp;

            if b >= self.from {
                println!("-->\t{}", a)
            }
            if b >= self.to {
                break;
            }
        }
    }
}
