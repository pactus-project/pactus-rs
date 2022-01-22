pub mod commands;
pub mod file;

pub use commands::Command;

fn main() {
    match Command::from_args() {
        Command::Init(cmd) => println!("{:?}", cmd.execute()),
        Command::Start(cmd) => println!("{:?}", cmd.execute()),
    }
}
