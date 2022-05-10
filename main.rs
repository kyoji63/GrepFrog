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


fn hello(name: Option<String>) -> String {
    return format!("Hello, {}", if let Some(n) = name {
        n
    } else {
        "World".to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!("Hello, World", hello(None));
        assert_eq!("Hello, Tamada", hello(Some("Tamada".to_string())));
    }
}