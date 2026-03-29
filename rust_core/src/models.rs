use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

/// TimeLocky CreateStream instruction data layout
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Serialize, Deserialize)]
pub struct CreateStreamParams {
    /// Timestamp when the stream starts (unix seconds)
    pub start_time: u64,
    /// Total deposited amount (in token smallest unit)
    pub net_amount_deposited: u64,
    /// Duration of one period in seconds
    pub period: u64,
    /// Amount released per period
    pub amount_per_period: u64,
    /// Cliff timestamp (unix seconds)
    pub cliff: u64,
    /// Amount released at cliff
    pub cliff_amount: u64,
    /// Whether sender can cancel
    pub cancelable_by_sender: bool,
    /// Whether recipient can cancel
    pub cancelable_by_recipient: bool,
    /// Auto-withdrawal enabled
    pub automatic_withdrawal: bool,
    /// Whether sender can transfer the stream
    pub transferable_by_sender: bool,
    /// Whether recipient can transfer the stream
    pub transferable_by_recipient: bool,
    /// Whether the stream can be topped up
    pub can_topup: bool,
    /// Stream name (padded to 64 bytes)
    pub stream_name: [u8; 64],
    /// Minimum withdraw frequency in seconds
    pub withdraw_frequency: u64,
    /// Whether the stream can be paused
    pub pausable: bool,
    /// Whether the release rate can be updated
    pub can_update_rate: bool,
}

/// On-chain stream account data layout for deserialization
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Serialize, Deserialize)]
pub struct StreamData {
    /// Magic number / version
    pub magic: u64,
    /// Created at timestamp
    pub created_at: u64,
    /// Withdrawn amount
    pub withdrawn_amount: u64,
    /// Cancelled at timestamp (0 if active)
    pub canceled_at: u64,
    /// End time
    pub end_time: u64,
    /// Last withdrawn at
    pub last_withdrawn_at: u64,
    /// Sender pubkey (32 bytes)
    pub sender: [u8; 32],
    /// Sender tokens account (32 bytes)
    pub sender_tokens: [u8; 32],
    /// Recipient pubkey (32 bytes)
    pub recipient: [u8; 32],
    /// Recipient tokens account (32 bytes)
    pub recipient_tokens: [u8; 32],
    /// Mint pubkey (32 bytes)
    pub mint: [u8; 32],
    /// Escrow tokens account (32 bytes)
    pub escrow_tokens: [u8; 32],
    /// TimeLocky treasury (32 bytes)
    pub timelocky_treasury: [u8; 32],
    /// TimeLocky treasury tokens (32 bytes)
    pub timelocky_treasury_tokens: [u8; 32],
    /// TimeLocky fee total
    pub timelocky_fee_total: u64,
    /// TimeLocky fee withdrawn
    pub timelocky_fee_withdrawn: u64,
    /// TimeLocky fee percent
    pub timelocky_fee_percent: f32,
    /// Partner fee total
    pub partner_fee_total: u64,
    /// Partner fee withdrawn
    pub partner_fee_withdrawn: u64,
    /// Partner fee percent
    pub partner_fee_percent: f32,
    /// Partner pubkey (32 bytes)
    pub partner: [u8; 32],
    /// Partner tokens (32 bytes)
    pub partner_tokens: [u8; 32],
    /// Start time
    pub start_time: u64,
    /// Net amount deposited
    pub net_amount_deposited: u64,
    /// Period in seconds
    pub period: u64,
    /// Amount per period
    pub amount_per_period: u64,
    /// Cliff timestamp
    pub cliff: u64,
    /// Cliff amount
    pub cliff_amount: u64,
    /// Cancelable by sender
    pub cancelable_by_sender: bool,
    /// Cancelable by recipient
    pub cancelable_by_recipient: bool,
    /// Automatic withdrawal
    pub automatic_withdrawal: bool,
    /// Transferable by sender
    pub transferable_by_sender: bool,
    /// Transferable by recipient
    pub transferable_by_recipient: bool,
    /// Can topup
    pub can_topup: bool,
    /// Stream name (64 bytes)
    pub stream_name: [u8; 64],
    /// Withdraw frequency
    pub withdraw_frequency: u64,
    /// Closed
    pub closed: bool,
    /// Current pause start
    pub current_pause_start: u64,
    /// Pause cumulative
    pub pause_cumulative: u64,
    /// Last rate change time
    pub last_rate_change_time: u64,
    /// Funds unlocked at last rate change
    pub funds_unlocked_at_last_rate_change: u64,
}

/// TimeLocky instruction discriminators
#[derive(Debug, Clone, Copy)]
pub enum TimeLockyInstruction {
    CreateStream = 0,
    Withdraw = 1,
    Cancel = 2,
    TransferRecipient = 3,
    TopUp = 4,
}
