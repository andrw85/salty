use passwords::PasswordGenerator;
use super::options;

pub fn gen(parameters : options::Opt) -> Result<String, &'static str> { 
    let options::Opt::Generator(opt) = parameters;
    let pg  = PasswordGenerator {
            length: opt.length,
            numbers: opt.numbers,
            lowercase_letters: opt.lowercase_letters,
            uppercase_letters: opt.uppercase_letters,
            symbols: opt.symbols,
            spaces: opt.spaces,
            exclude_similar_characters: opt.exclude_similar_characters,
            strict: opt.strict,
        };

    pg.generate_one()
}        
