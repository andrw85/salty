use salty::vault::*;

#[test]
fn test_account_add() {


    let mut account = Account::new();
    let entry = AccountEntry::new("google", "andrew", "123456789");
    let entry2 = AccountEntry::new("amazon", "andrew", "123456789");
    let entry3 = AccountEntry::new("facebook", "andrew", "123456789");

    assert!(account.add(entry).is_ok());
    assert!(account.add(entry2).is_ok());
    assert!(account.add(entry3).is_ok());

    let entry_already_there = AccountEntry::new("facebook", "andrew", "123456789");
    assert!(account.add(entry_already_there).is_err());
    
    assert_eq!(account.size(), 3);
}


    // let pwd = "Hello";
    // let hasher_salt = "xxxx";
    // let pw_hashed = hasher::hash(&pwd, &hasher_salt).unwrap();