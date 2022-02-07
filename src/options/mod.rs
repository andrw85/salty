pub use structopt::{StructOpt};

pub fn options() -> Opt {
    Opt::build()
}
/// yaovault (Yet Another Open Vault) is an open implementation of a password management system.
#[derive(StructOpt, Debug)]
#[structopt(name = env!("CARGO_PKG_NAME"))]
#[structopt(version = env!("CARGO_PKG_VERSION"))]
#[structopt(about = env!("CARGO_PKG_DESCRIPTION"))]
#[structopt(author = env!("CARGO_PKG_AUTHORS"))]
pub enum Opt {
    Generator(PasswordGenOpt),
}

/// A password generation tool
#[derive(StructOpt, Debug)]
pub struct PasswordGenOpt {
    #[structopt(long, default_value = "8")]
    pub length: usize,
    #[structopt(long)]
    pub numbers: bool,
    #[structopt(short = "lc", long = "lowercase")]
    pub lowercase_letters: bool,
    #[structopt(short = "u", long = "uppercase")]
    pub uppercase_letters: bool,
    #[structopt(long)]
    pub symbols: bool,
    #[structopt(long)]
    pub spaces: bool,
    #[structopt(long)]
    pub exclude_similar_characters: bool,
    #[structopt(long)]
    pub strict: bool,
    #[structopt(long, default_value = "aot/I3YepRSH5AaZe+oDEQ")]
    pub hasher_salt: String,
    #[structopt(long, required_unless = "numbers, lowercase_letters, uppercase_letters, symbols, spaces")]
    pub default: bool,
}

impl Opt {
    fn build() -> Self {
        let opt = Self::from_args();
        if let Opt::Generator(o) = opt {
            if o.default {    
                // let out = PasswordGenOpt {
                //     numbers : true,
                //     lowercase_letters : true,
                //     uppercase_letters : true,
                //     symbols : true,
                //     exclude_similar_characters : true,
                //     strict : true,
                //     spaces : false,
                //     default : true,
                //     hasher_salt : opt.Generator.hasher_salt,
                //     length :  opt.length,
                // };      
                return Opt::Generator(PasswordGenOpt{
                    numbers : true,
                    lowercase_letters : true,
                    uppercase_letters : true,
                    symbols : true,
                    exclude_similar_characters : true,
                    strict : true,
                    spaces : false,
                    default : true,
                    hasher_salt: o.hasher_salt,
                    length: o.length,
                })
            }
            return Opt::Generator(o)
        }
        opt
    }
}
