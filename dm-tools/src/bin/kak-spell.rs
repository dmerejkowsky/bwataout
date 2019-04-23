use regex::Regex;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "rspell", about = "Spell checker backend for Kakoune")]
pub struct Opts {
    #[structopt(long = "kakoune", help = "produce output readable by kakoune")]
    pub kakoune: bool,

    #[structopt(long = "timestamp", help = "buffer timestamp")]
    pub timestamp: u32,

    #[structopt(long = "lang", help = "language to use")]
    pub lang: String,

    #[structopt(help = "buffer name")]
    pub bufname: String,
}

fn main() {
    let opts = Opts::from_args();
    let mut broker = enchant::Broker::new();
    let dict = broker.request_dict(&opts.lang).unwrap();

    let timestamp = &opts.timestamp;
    let contents =
        std::fs::read_to_string(&opts.bufname).expect(&format!("could not open {}", &opts.bufname));
    for (lineno, line) in contents.lines().enumerate() {
        let tokens = tokenize(line);
        for token in tokens {
            let checked = dict.check(token.text).unwrap();
            if !checked {
                let Token { start, end, .. } = token;
                let region = format!(
                    "{lineno}.{start},{lineno}.{end}",
                    lineno = lineno + 1,
                    start = start + 1,
                    end = end,
                );
                println!(
                    "set-option buffer spell_regions {} {}|white,red",
                    timestamp, region
                );
            }
        }
    }
}

#[derive(Debug)]
struct Token<'a> {
    text: &'a str,
    start: usize,
    end: usize,
}

fn tokenize(text: &str) -> Vec<Token> {
    let re = Regex::new(r"\p{Letter}+").unwrap(); // unicode rules :)
    let mut res = vec![];
    for chunk in re.find_iter(text) {
        let start = chunk.start();
        let end = chunk.end();
        let token = Token {
            text: chunk.as_str(),
            start,
            end,
        };
        res.push(token);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let text = "this is some words";
        let tokens = tokenize(text);
        dbg!(&tokens);
        assert_eq!(tokens.len(), 4);

        let french_text = "J'ai dit: «J'aime le café»";
        let tokens = tokenize(french_text);
        dbg!(&tokens);
    }

}
