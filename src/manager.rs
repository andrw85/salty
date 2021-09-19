mod options;
mod password;
mod hasher;
mod storage;

use options::options;
use storage::{Account, AccountEntry};

fn main() {
    let opt = options();       
    // let pw = password::gen(opt).unwrap();
    let options::Opt::Manager(ManagerOpt(site, user, pwd)) = opt;

    let pw_hashed = hasher::hash(&pwd, &hasher_salt).unwrap();

    let mut account = Account::new(&pw_hashed);
    let entry = AccountEntry::new("google", "andrew", "123456789");
    let entry2 = AccountEntry::new("amazon", "andrew", "123456789");
    let entry3 = AccountEntry::new("facebook", "andrew", "123456789");

    account.add(entry);
    account.add(entry2);
    account.add(entry3);

    if let Ok(account2) = Account::load_from_file(&pw_hashed) {
        println!("{:#?}",account2);
    } else {
        println!("failed!");
    }
}
