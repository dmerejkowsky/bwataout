use regex::Regex;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "rspell", about = "Spell checker backend for Kakoune")]
struct Opts {
    #[structopt(long = "lang", help = "language to use")]
    lang: String,

    #[structopt(subcommand)]
    sub_cmd: SubCommand,
}

#[derive(StructOpt)]
enum SubCommand {
    #[structopt(name = "check", about = "Spell check the current buffer")]
    Check {
        #[structopt(long = "timestamp", help = "buffer timestamp")]
        timestamp: usize,

        #[structopt(help = "buffer name")]
        filename: String,
    },

    #[structopt(name = "add", about = "Add word under selection to personal dict")]
    Add {
        #[structopt(help = "selection")]
        selection: String,
    },
}

fn check(dict: enchant::Dict, filename: &str, timestamp: usize) {
    let contents =
        std::fs::read_to_string(filename).expect(&format!("could not open {}", filename));
    let mut spell_regions = String::new();
    for (lineno, line) in contents.lines().enumerate() {
        let tokens = tokenize(line);
        for token in tokens {
            let checked = dict.check(token.text).unwrap();
            if !checked {
                let Token { start, end, .. } = token;
                let region = format!(
                    "{lineno}.{start},{lineno}.{end}|white,red ",
                    lineno = lineno + 1,
                    start = start + 1,
                    end = end,
                );
                spell_regions.push_str(&region);
            }
        }
    }
    println!(
        "set-option buffer spell_regions {} {}",
        timestamp, spell_regions,
    );
}

fn main() {
    let opts = Opts::from_args();
    let mut broker = enchant::Broker::new();
    let lang = &opts.lang;
    let dict = broker.request_dict(lang).unwrap();

    match &opts.sub_cmd {
        SubCommand::Check {
            timestamp,
            filename,
        } => check(dict, filename, *timestamp),
        SubCommand::Add { selection } => dict.add(selection),
    };
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
