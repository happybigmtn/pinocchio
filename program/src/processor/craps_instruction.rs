use shank::ShankInstruction;

#[derive(ShankInstruction)]
pub enum CrapsInstruction {
    InitializeSystem {
        rng_authority: Pubkey,
    },
    InitializePlayer,
    Deposit {
        amount: u64,
    },
    Withdraw {
        amount: u64,
    },
    PlaceBet {
        epoch: u64,
        bet_kind: u8,
        bet_amount: u64,
    },
    RollDice,
    SettleBets {
        epochs: Vec<u64>,
    },
    PauseGame {
        pause: bool,
    },
    UpdateAuthority {
        new_authority: Pubkey,
    },
}
