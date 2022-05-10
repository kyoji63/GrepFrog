use clap::Parser;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(
        name = "grepfrog",
        version = "1.0.0", 
        about = ""grepfrog 🐸" is a command that aggregates file modification commands. I will gradually add more functions...",
)]

struct Args{
    /// Name of the person to greet
    #[clap(c, change)]
    name: String,

    #[clap(e, encode)]
    name: String,

    String1 : String,
    String2 : String,
}

fn main(){
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_method1() {
        assert_eq!(...);
    }
}
