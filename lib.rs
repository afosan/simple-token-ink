#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod simple_token_ink {
    use ink::storage::Mapping;

    #[ink(storage)]
    #[derive(Default)]
    pub struct SimpleToken {
        balances: Mapping<AccountId, Balance>,
        total_supply: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
    }

    impl SimpleToken {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller();
            balances.insert(caller, &total_supply);
            Self { balances, total_supply }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> Balance {
            self.balances.get(&account).unwrap_or_default()
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<(), Error> {
            let from = self.env().caller();
            let from_balance = self.balance_of(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }
            let to_balance = self.balance_of(to);

            self.balances.insert(from, &(from_balance - value));
            self.balances.insert(to, &(to_balance + value));

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn total_supply_works() {
            let simple_token = SimpleToken::new(100);
            assert_eq!(simple_token.total_supply(), 100);
        }

        #[ink::test]
        fn balance_of_works() {
            let simple_token = SimpleToken::new(100);
            let accounts =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(simple_token.balance_of(accounts.alice), 100);
            assert_eq!(simple_token.balance_of(accounts.bob), 0);
        }

        #[ink::test]
        fn transfer_works() {
            let mut simple_token = SimpleToken::new(100);
            let accounts =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(simple_token.balance_of(accounts.bob), 0);
            assert_eq!(simple_token.transfer(accounts.bob, 10), Ok(()));
            assert_eq!(simple_token.balance_of(accounts.bob), 10);
        }
    }
}
