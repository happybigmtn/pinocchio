use crate::processor::craps_instruction::CrapsInstruction::*;
use pinocchio::{account_info::AccountInfo, msg, pubkey::Pubkey, ProgramResult};

pub fn dispatch_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction =
        try_from_slice(instruction_data).map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        InitializeSystem { rng_authority } => {
            processor::initialize_system(program_id, accounts, rng_authority)
        }
        InitializePlayer => processor::initialize_player(program_id, accounts),
        Deposit { amount } => processor::deposit(program_id, accounts, amount),
        Withdraw { amount } => processor::withdraw(program_id, accounts, amount),
        PlaceBet {
            epoch,
            bet_kind,
            bet_amount,
        } => processor::place_bet(
            program_id,
            accounts,
            epoch,
            bet_kind,
            bet_amount,
            repeater_target,
        ),
        RollDice => processor::roll_dice(program_id, accounts),
        SettleBets { epochs } => processor::settle_bets(program_id, accounts, epochs),
        PauseGame { pause } => processor::pause_game(program_id, accounts, pause),
        UpdateAuthority { new_authority } => {
            processor::update_authority(program_id, accounts, new_authority)
        }
    }
}
