use borsh::from_slice;
use solana_program::program_error::ProgramError;

pub enum CounterInstruction {
    Increment(u32),
    Decrement(u32),
    Update(u32),
    Reset,
}

impl CounterInstruction {
    pub const INCREMENT: u8 = 0;
    pub const DECREMENT: u8 = 1;
    pub const UPDATE: u8 = 2;
    pub const RESET: u8 = 3;

    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            Self::INCREMENT => Self::Increment(from_slice(rest).unwrap()),
            Self::DECREMENT => Self::Decrement(from_slice(rest).unwrap()),
            Self::UPDATE => Self::Update(from_slice(rest).unwrap()),
            Self::RESET => Self::Reset,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
