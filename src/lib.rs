use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

const PASSWORD_NUMBER: u8 = 1;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // SETUP CONTRACT STATE
    password_solution: String,
}

#[near_bindgen]
impl Contract {
    // ADD CONTRACT METHODS HERE
    pub fn get_password_number(&self) -> u8 {
        PASSWORD_NUMBER
    }

    pub fn set_solution(&mut self, solution: String) {
        self.password_solution = solution;
    }

    pub fn guess_solution(&mut self, solution: String) {
        if solution == self.password_solution {
            env::log_str("You may enter! This is the right password");
        } else {
            env::log_str("You may enter! This is the wrong password");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
}
