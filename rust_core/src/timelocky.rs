use borsh::BorshSerialize;
use sha2::{Digest, Sha256};

use crate::models::{CreateStreamParams, StreamData, TimeLockyInstruction};

/// TimeLocky Program ID (mainnet)
pub const TIMELOCKY_PROGRAM_ID: &str = "strmRqUCoQUgGUFGbELMFXBYfhMtR6psGfYHxy3BEiZ";

/// TimeLocky treasury
pub const TIMELOCKY_TREASURY: &str = "5SEpbdjFK5FxwTvfsQMYyEr2T2AccFYEJxFGPauDrkMy";

/// SPL Token Program ID
pub const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

/// Associated Token Program ID
pub const ASSOCIATED_TOKEN_PROGRAM_ID: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";

/// System Program ID
pub const SYSTEM_PROGRAM_ID: &str = "11111111111111111111111111111111";

/// Rent Sysvar
pub const RENT_SYSVAR: &str = "SysvarRent111111111111111111111111111111111";

/// Build the Sighash anchor discriminator for a given instruction name
pub fn sighash(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{namespace}:{name}");
    let mut hasher = Sha256::new();
    hasher.update(preimage.as_bytes());
    let hash = hasher.finalize();
    let mut disc = [0u8; 8];
    disc.copy_from_slice(&hash[..8]);
    disc
}

/// Build serialized instruction data for CreateStream
pub fn build_create_stream_ix_data(params: &CreateStreamParams) -> Vec<u8> {
    let disc = sighash("global", "create");
    let mut data = disc.to_vec();
    let serialized = params.try_to_vec().expect("borsh serialization failed");
    data.extend_from_slice(&serialized);
    data
}

/// Build serialized instruction data for Withdraw
pub fn build_withdraw_ix_data(amount: u64) -> Vec<u8> {
    let disc = sighash("global", "withdraw");
    let mut data = disc.to_vec();
    data.extend_from_slice(&amount.to_le_bytes());
    data
}

/// Build serialized instruction data for Cancel
pub fn build_cancel_ix_data() -> Vec<u8> {
    let disc = sighash("global", "cancel");
    disc.to_vec()
}

/// Build serialized instruction data for TransferRecipient
pub fn build_transfer_ix_data() -> Vec<u8> {
    let disc = sighash("global", "transfer_recipient");
    disc.to_vec()
}

/// Build serialized instruction data for TopUp
pub fn build_topup_ix_data(amount: u64) -> Vec<u8> {
    let disc = sighash("global", "topup");
    let mut data = disc.to_vec();
    data.extend_from_slice(&amount.to_le_bytes());
    data
}

/// Pad a stream name to 64 bytes
pub fn pad_stream_name(name: &str) -> [u8; 64] {
    let mut padded = [0u8; 64];
    let bytes = name.as_bytes();
    let len = bytes.len().min(64);
    padded[..len].copy_from_slice(&bytes[..len]);
    padded
}

/// Deserialize on-chain stream account data
pub fn decode_stream_data(data: &[u8]) -> Result<StreamData, String> {
    // Skip the 8-byte discriminator
    if data.len() < 8 {
        return Err("Data too short".to_string());
    }
    borsh::from_slice::<StreamData>(&data[8..])
        .map_err(|e| format!("Failed to deserialize stream data: {e}"))
}
