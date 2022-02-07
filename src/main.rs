mod options;
mod password;
mod hasher;

use options::options;

fn main() {
    let opt = options();       
    // let pw = password::gen(opt).unwrap();
    let options::Opt::Generator(options::PasswordGenOpt{ hasher_salt, .. }) = opt;

    let pw = "holahola".to_string() + &hasher_salt;
    let pw_hashed = hasher::hash(&pw, &hasher_salt).unwrap();
    
    println!("pwd: {:?}", &pw);
    println!("hash: {:?}", &pw_hashed);
}
