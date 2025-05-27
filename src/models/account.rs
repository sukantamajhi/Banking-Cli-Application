use crate::models::user::User;

#[derive(Debug, Clone)]
pub struct Account {
    pub id: u32,
    pub balance: i32,
    pub account_number: u32,
    pub holder: User,
}

impl Account {
    pub fn new(holder: User) -> Self {
        Self {
            id: 0, // TODO: Implement proper ID generation
            balance: 0,
            account_number: holder.account_number,
            holder,
        }
    }

    pub fn deposit(&mut self, amount: i32) -> Result<(), String> {
        if amount <= 0 {
            return Err("Deposit amount must be positive".to_string());
        }
        self.balance += amount;
        Ok(())
    }

    pub fn withdraw(&mut self, amount: i32) -> Result<(), String> {
        if amount <= 0 {
            return Err("Withdrawal amount must be positive".to_string());
        }
        if self.balance < amount {
            return Err("Insufficient funds".to_string());
        }
        self.balance -= amount;
        Ok(())
    }

    pub fn get_balance(&self) -> i32 {
        self.balance
    }
}
