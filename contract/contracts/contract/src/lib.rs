#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, Address};

#[contract]
pub struct PiggyBank;

#[contractimpl]
impl PiggyBank {

    // Khởi tạo owner
    pub fn init(env: Env, owner: Address) {
        let key = Symbol::short("OWNER");
        env.storage().instance().set(&key, &owner);
    }

    // Nạp tiền (giả lập tăng balance)
    pub fn deposit(env: Env, amount: i128) {
        let key = Symbol::short("BAL");
        let mut balance: i128 = env.storage().instance().get(&key).unwrap_or(0);
        balance += amount;
        env.storage().instance().set(&key, &balance);
    }

    // Rút tiền (chỉ owner)
    pub fn withdraw(env: Env, to: Address, amount: i128) {
        let owner_key = Symbol::short("OWNER");
        let balance_key = Symbol::short("BAL");

        let owner: Address = env.storage().instance().get(&owner_key).unwrap();
        owner.require_auth(); // xác thực owner

        let mut balance: i128 = env.storage().instance().get(&balance_key).unwrap_or(0);

        if balance < amount {
            panic!("Not enough balance");
        }

        balance -= amount;
        env.storage().instance().set(&balance_key, &balance);

        // (Trong thực tế sẽ transfer token, ở đây chỉ demo logic)
    }

    // Xem số dư
    pub fn get_balance(env: Env) -> i128 {
        let key = Symbol::short("BAL");
        env.storage().instance().get(&key).unwrap_or(0)
    }
}
