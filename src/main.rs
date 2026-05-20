#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank {
            accounts: Vec::new(),
        }
    }
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            balance: 0,
            holder,
        }
    }
}

fn print_account(account: Account) {
    println!("{:#?}", account);
}

fn print_bank(bank: Bank) {
    println!("{:#?}", bank);
}

fn main() {
    let bank = Bank::new();
    let account = Account::new(1, String::from("Anton"));

    print_bank(bank);
    print_account(account);
}
