pub mod instruction;
pub mod process;
pub mod state;

use process::process_instructions;
use solana_program::entrypoint;

entrypoint!(process_instructions);

#[cfg(test)]
mod test {
    use crate::instruction::CounterInstruction;
    use crate::process::process_instructions;
    use crate::state::Counter;
    use borsh::BorshDeserialize;
    use solana_program::clock::Epoch;
    use solana_sdk::{account_info::AccountInfo, pubkey::Pubkey};
    use std::mem;

    #[test]
    fn test_counter() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();

        let counter_account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );

        let accounts = vec![counter_account];

        // increment
        let increment_value: u32 = 10;
        let mut increment_instruction_data = vec![CounterInstruction::INCREMENT];
        increment_instruction_data.extend_from_slice(&increment_value.to_le_bytes());

        process_instructions(&program_id, &accounts, &increment_instruction_data).unwrap();
        assert_eq!(
            Counter::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            increment_value
        );

        // decrement
        let decrement_value: u32 = 12;
        let mut decremnet_instruction_data = vec![CounterInstruction::DECREMENT];
        decremnet_instruction_data.extend_from_slice(&decrement_value.to_le_bytes());

        process_instructions(&program_id, &accounts, &decremnet_instruction_data).unwrap();
        assert_eq!(
            Counter::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            if increment_value > decrement_value {
                increment_value - decrement_value
            } else {
                0
            }
        );

        // update
        let update_value = 12u32;
        let mut update_instruction_data = vec![CounterInstruction::UPDATE];
        update_instruction_data.extend_from_slice(&update_value.to_le_bytes());

        process_instructions(&program_id, &accounts, &update_instruction_data).unwrap();
        assert_eq!(
            Counter::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            update_value
        );

        // reset
        let reset_instruction_data = vec![CounterInstruction::RESET];

        process_instructions(&program_id, &accounts, &reset_instruction_data).unwrap();
        assert_eq!(
            Counter::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
    }
}
