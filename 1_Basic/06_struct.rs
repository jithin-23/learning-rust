fn main() {
    let mut account = BankAccount{
        owner: "Alice".to_string(),
        balance: 1000.0
    };

    //Immutable borrow to check balance
    account.check_balance();

    //mutable borrow to withdraw money
    account.withdraw(200.0);
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!(
            "Withdrawing {} from account owned by {}",
            amount, self.owner
        );
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println! {"Account owned by {} has balance {}", self.owner, self.balance};
    }
}
