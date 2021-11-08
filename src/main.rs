
mod cli;

pub use cli::Cli;

fn main() {
    let cli = Cli::new();
    
    println!("{}", cli);
}
