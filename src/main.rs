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

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) ->i32 {
        self.accounts
        .iter()
        .map(|account| account.balance)
        .sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts.iter()
        .map(|account| account.summary())
        .collect::<Vec<String>>()
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

    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
    }

    fn summary(&self) -> String {
        format!("Account: {} with balance: {}", self.holder, self.balance)
    }
}


fn main() {
    let mut bank = Bank::new();

    let mut account = Account::new(1, String::from("Anton"));

    account.deposit(100);
    account.withdraw(50);

    bank.add_account(account);
    println!("{:#?}", bank.summary());
    println!("Total balance: {}", bank.total_balance());
}
