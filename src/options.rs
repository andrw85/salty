pub use structopt::{StructOpt};

pub fn options() -> Opt {
    Opt::build()
}
/// Salty  is an open implementation of a password management system.
#[derive(StructOpt, Debug)]
#[structopt(name = env!("CARGO_PKG_NAME"))]
#[structopt(version = env!("CARGO_PKG_VERSION"))]
#[structopt(about = env!("CARGO_PKG_DESCRIPTION"))]
#[structopt(author = env!("CARGO_PKG_AUTHORS"))]
pub enum Opt {
    Generator(PasswordGenOpt),
    CreateVault,
    AddSite(AddSiteOpt),
    ShowEntries,
}
#[derive(StructOpt, Debug)]
pub struct FlagsOpt {
    #[structopt(long, default_value = "aot/I3YepRSH5AaZe+oDEQ")]
    pub hasher_salt: String,
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

#[derive(StructOpt, Debug)]
pub enum ManagerOpt {
    AddSite(AddSiteOpt),
}

#[derive(StructOpt, Debug)]
pub struct AddSiteOpt {
    #[structopt(short, long, required = true)]
    pub site: String,
    #[structopt(short, long, required = true)]
    pub user: String,
    #[structopt(long, default_value = "aot/I3YepRSH5AaZe+oDEQ")]
    pub hasher_salt: String,
}

impl Opt {
    fn build() -> Self {
        let opt = Self::from_args();
        if let Opt::Generator(o) = opt {
            if o.default {     
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
