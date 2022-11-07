#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod mytoken {
    use ink_storage::{traits::SpreadAllocate, Mapping};

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Mytoken {
        total_supply: u32,
        balances: Mapping<AccountId, u32>,
    }

    use ink_lang::utils::initialize_contract;
    impl Mytoken {
        /// Creates a token contract with the given initial supply belonging to the contract creator.
        #[ink(constructor)]
        pub fn new_token(supply: u32) -> Self {
            initialize_contract(|contract: &mut Self| {
                let caller = Self::env().caller();
                contract.balances.insert(&caller, &supply);
                contract.total_supply = supply;
            })
        }

        /// Total supply of the token.
        #[ink(message)]
        pub fn total_supply(&self) -> u32 {
            self.total_supply
        }

        /// Current balance of the chosen account.
        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> u32 {
            match self.balances.get(&account) {
                Some(value) => value,
                None => 0,
            }
        }

        /// Transfers an amount of tokens to the chosen recipient.
        #[ink(message)]
        pub fn transfer(&mut self, recipient: AccountId, amount: u32) {
            let sender = self.env().caller();
            let sender_balance = self.balance_of(sender);
            if sender_balance < amount {
                return;
            }
            self.balances.insert(sender, &(sender_balance - amount));
            let recipient_balance = self.balance_of(recipient);
            self.balances.insert(recipient, &(recipient_balance + amount));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mytoken::Mytoken;
    use ink_env::{test, DefaultEnvironment};
    use ink_lang as ink;

    #[ink::test]
    fn total_supply_works() {
        let mytoken = Mytoken::new_token(1000);
        assert_eq!(mytoken.total_supply(), 1000);
    }

    #[ink::test]
    fn balance_of_works() {
        let accounts = test::default_accounts::<DefaultEnvironment>();
        test::set_caller::<DefaultEnvironment>(accounts.alice);
        let mytoken = Mytoken::new_token(1000);
        assert_eq!(mytoken.balance_of(accounts.alice), 1000);
        assert_eq!(mytoken.balance_of(accounts.bob), 0);
    }

    #[ink::test]
    fn transfer_works() {
        let accounts = test::default_accounts::<DefaultEnvironment>();
        test::set_caller::<DefaultEnvironment>(accounts.alice);
        let mut mytoken = Mytoken::new_token(1000);
        assert_eq!(mytoken.balance_of(accounts.alice), 1000);
        assert_eq!(mytoken.balance_of(accounts.bob), 0);
        mytoken.transfer(accounts.bob, 100);
        assert_eq!(mytoken.balance_of(accounts.alice), 900);
        assert_eq!(mytoken.balance_of(accounts.bob), 100);
    }
}