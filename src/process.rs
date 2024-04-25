use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::{instruction::CounterInstruction, state::Counter};

pub fn process_instructions(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Counter Program!");

    // instruction
    let instruction = CounterInstruction::unpack(instruction_data)?;

    // counter account
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    let mut counter_account = Counter::try_from_slice(&account.data.borrow())?;

    match instruction {
        CounterInstruction::Increment(value) => {
            counter_account.counter += value;
        }
        CounterInstruction::Decrement(value) => {
            counter_account.counter = if counter_account.counter > value {
                counter_account.counter - value
            } else {
                0
            };
        }
        CounterInstruction::Update(value) => {
            counter_account.counter = value;
        }
        CounterInstruction::Reset => {
            counter_account.counter = 0;
        }
    }

    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}
