use std::env;
use std::fmt;
use std::result::Result;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 5 {
        if let (Ok(amount), Ok(from), Ok(to)) = (
            args[1].parse::<f32>(),
            Currency::from_str(&args[2]),
            Currency::from_str(&args[4]),
        ) {
            println!(
                "{} {} are {} {}",
                amount,
                from,
                amount / from.value() * to.value(),
                to
            );
        }
    }
}

#[derive(Debug, Clone)]
enum Currency {
    EUR,
    USD,
    JPY,
    GBP,
}

impl Currency {
    fn value(&self) -> f32 {
        match self {
            Self::EUR => 1.0,
            Self::USD => 1.21562,
            Self::JPY => 127.85,
            Self::GBP => 0.86285,
        }
    }
}

#[derive(Debug, Clone)]
struct ParseCurrencyError;

impl FromStr for Currency {
    type Err = ParseCurrencyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_ref() {
            "eur" => Ok(Self::EUR),
            "usd" => Ok(Self::USD),
            "gbp" => Ok(Self::GBP),
            "jpy" => Ok(Self::JPY),
            _ => Err(ParseCurrencyError),
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
