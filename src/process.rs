pub mod handler;

use borsh::{BorshDeserialize, BorshSerialize};
use handler::{handle_decrement, handle_increment, handle_reset, handle_update};
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
        CounterInstruction::Increment(value) => handle_increment(&mut counter_account, value),
        CounterInstruction::Decrement(value) => handle_decrement(&mut counter_account, value),
        CounterInstruction::Update(value) => handle_update(&mut counter_account, value),
        CounterInstruction::Reset => handle_reset(&mut counter_account),
    }

    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}
