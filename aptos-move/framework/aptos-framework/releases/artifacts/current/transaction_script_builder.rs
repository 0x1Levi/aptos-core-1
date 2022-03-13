// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

// This file was generated. Do not modify!
//
// To update this code, run: `cargo run --release -p framework`.

//! Conversion library between a structured representation of a Move script call (`ScriptCall`) and the
//! standard BCS-compatible representation used in Aptos transactions (`Script`).
//!
//! This code was generated by compiling known Script interfaces ("ABIs") with the tool `transaction-builder-generator`.

#![allow(clippy::unnecessary_wraps)]
#![allow(unused_imports)]
use aptos_types::{
    account_address::AccountAddress,
    transaction::{Script, ScriptFunction, TransactionArgument, TransactionPayload, VecBytes},
};
use move_core_types::{
    ident_str,
    language_storage::{ModuleId, TypeTag},
};
use std::collections::BTreeMap as Map;

type Bytes = Vec<u8>;

/// Structured representation of a call into a known Move script function.
/// ```ignore
/// impl ScriptFunctionCall {
///     pub fn encode(self) -> TransactionPayload { .. }
///     pub fn decode(&TransactionPayload) -> Option<ScriptFunctionCall> { .. }
/// }
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "fuzzing", derive(proptest_derive::Arbitrary))]
#[cfg_attr(feature = "fuzzing", proptest(no_params))]
pub enum ScriptFunctionCall {
    ClaimMintCapability {},

    CreateAccount {
        new_account_address: AccountAddress,
        auth_key_prefix: Bytes,
    },

    DelegateMintCapability {
        addr: AccountAddress,
    },

    Mint {
        addr: AccountAddress,
        amount: u64,
    },

    RotateAuthenticationKey {
        new_authentication_key: Bytes,
    },

    SetGasConstants {
        global_memory_per_byte_cost: u64,
        global_memory_per_byte_write_cost: u64,
        min_transaction_gas_units: u64,
        large_transaction_cutoff: u64,
        intrinsic_gas_per_byte: u64,
        maximum_number_of_gas_units: u64,
        min_price_per_gas_unit: u64,
        max_price_per_gas_unit: u64,
        max_transaction_size_in_bytes: u64,
        gas_unit_scaling_factor: u64,
        default_account_size: u64,
    },

    Transfer {
        to: AccountAddress,
        amount: u64,
    },
}

impl ScriptFunctionCall {
    /// Build an Aptos `TransactionPayload` from a structured object `ScriptFunctionCall`.
    pub fn encode(self) -> TransactionPayload {
        use ScriptFunctionCall::*;
        match self {
            ClaimMintCapability {} => encode_claim_mint_capability_script_function(),
            CreateAccount {
                new_account_address,
                auth_key_prefix,
            } => encode_create_account_script_function(new_account_address, auth_key_prefix),
            DelegateMintCapability { addr } => {
                encode_delegate_mint_capability_script_function(addr)
            }
            Mint { addr, amount } => encode_mint_script_function(addr, amount),
            RotateAuthenticationKey {
                new_authentication_key,
            } => encode_rotate_authentication_key_script_function(new_authentication_key),
            SetGasConstants {
                global_memory_per_byte_cost,
                global_memory_per_byte_write_cost,
                min_transaction_gas_units,
                large_transaction_cutoff,
                intrinsic_gas_per_byte,
                maximum_number_of_gas_units,
                min_price_per_gas_unit,
                max_price_per_gas_unit,
                max_transaction_size_in_bytes,
                gas_unit_scaling_factor,
                default_account_size,
            } => encode_set_gas_constants_script_function(
                global_memory_per_byte_cost,
                global_memory_per_byte_write_cost,
                min_transaction_gas_units,
                large_transaction_cutoff,
                intrinsic_gas_per_byte,
                maximum_number_of_gas_units,
                min_price_per_gas_unit,
                max_price_per_gas_unit,
                max_transaction_size_in_bytes,
                gas_unit_scaling_factor,
                default_account_size,
            ),
            Transfer { to, amount } => encode_transfer_script_function(to, amount),
        }
    }

    /// Try to recognize an Aptos `TransactionPayload` and convert it into a structured object `ScriptFunctionCall`.
    pub fn decode(payload: &TransactionPayload) -> Option<ScriptFunctionCall> {
        if let TransactionPayload::ScriptFunction(script) = payload {
            match SCRIPT_FUNCTION_DECODER_MAP.get(&format!(
                "{}{}",
                script.module().name(),
                script.function()
            )) {
                Some(decoder) => decoder(payload),
                None => None,
            }
        } else {
            None
        }
    }
}

pub fn encode_claim_mint_capability_script_function() -> TransactionPayload {
    TransactionPayload::ScriptFunction(ScriptFunction::new(
        ModuleId::new(
            AccountAddress::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
            ident_str!("AdminScripts").to_owned(),
        ),
        ident_str!("claim_mint_capability").to_owned(),
        vec![],
        vec![],
    ))
}

pub fn encode_create_account_script_function(
    new_account_address: AccountAddress,
    auth_key_prefix: Vec<u8>,
) -> TransactionPayload {
    TransactionPayload::ScriptFunction(ScriptFunction::new(
        ModuleId::new(
            AccountAddress::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
            ident_str!("BasicScripts").to_owned(),
        ),
        ident_str!("create_account").to_owned(),
        vec![],
        vec![
            bcs::to_bytes(&new_account_address).unwrap(),
            bcs::to_bytes(&auth_key_prefix).unwrap(),
        ],
    ))
}

pub fn encode_delegate_mint_capability_script_function(addr: AccountAddress) -> TransactionPayload {
    TransactionPayload::ScriptFunction(ScriptFunction::new(
        ModuleId::new(
            AccountAddress::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
            ident_str!("AdminScripts").to_owned(),
        ),
        ident_str!("delegate_mint_capability").to_owned(),
        vec![],
        vec![bcs::to_bytes(&addr).unwrap()],
    ))
}

pub fn encode_mint_script_function(addr: AccountAddress, amount: u64) -> TransactionPayload {
    TransactionPayload::ScriptFunction(ScriptFunction::new(
        ModuleId::new(
            AccountAddress::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
            ident_str!("AdminScripts").to_owned(),
        ),
        ident_str!("mint").to_owned(),
        vec![],
        vec![
            bcs::to_bytes(&addr).unwrap(),
            bcs::to_bytes(&amount).unwrap(),
        ],
    ))
}

pub fn encode_rotate_authentication_key_script_function(
    new_authentication_key: Vec<u8>,
) -> TransactionPayload {
    TransactionPayload::ScriptFunction(ScriptFunction::new(
        ModuleId::new(
            AccountAddress::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
            ident_str!("BasicScripts").to_owned(),
        ),
        ident_str!("rotate_authentication_key").to_owned(),
        vec![],
        vec![bcs::to_bytes(&new_authentication_key).unwrap()],
    ))
}

pub fn encode_set_gas_constants_script_function(
    global_memory_per_byte_cost: u64,
    global_memory_per_byte_write_cost: u64,
    min_transaction_gas_units: u64,
    large_transaction_cutoff: u64,
    intrinsic_gas_per_byte: u64,
    maximum_number_of_gas_units: u64,
    min_price_per_gas_unit: u64,
    max_price_per_gas_unit: u64,
    max_transaction_size_in_bytes: u64,
    gas_unit_scaling_factor: u64,
    default_account_size: u64,
) -> TransactionPayload {
    TransactionPayload::ScriptFunction(ScriptFunction::new(
        ModuleId::new(
            AccountAddress::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
            ident_str!("AdminScripts").to_owned(),
        ),
        ident_str!("set_gas_constants").to_owned(),
        vec![],
        vec![
            bcs::to_bytes(&global_memory_per_byte_cost).unwrap(),
            bcs::to_bytes(&global_memory_per_byte_write_cost).unwrap(),
            bcs::to_bytes(&min_transaction_gas_units).unwrap(),
            bcs::to_bytes(&large_transaction_cutoff).unwrap(),
            bcs::to_bytes(&intrinsic_gas_per_byte).unwrap(),
            bcs::to_bytes(&maximum_number_of_gas_units).unwrap(),
            bcs::to_bytes(&min_price_per_gas_unit).unwrap(),
            bcs::to_bytes(&max_price_per_gas_unit).unwrap(),
            bcs::to_bytes(&max_transaction_size_in_bytes).unwrap(),
            bcs::to_bytes(&gas_unit_scaling_factor).unwrap(),
            bcs::to_bytes(&default_account_size).unwrap(),
        ],
    ))
}

pub fn encode_transfer_script_function(to: AccountAddress, amount: u64) -> TransactionPayload {
    TransactionPayload::ScriptFunction(ScriptFunction::new(
        ModuleId::new(
            AccountAddress::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
            ident_str!("BasicScripts").to_owned(),
        ),
        ident_str!("transfer").to_owned(),
        vec![],
        vec![bcs::to_bytes(&to).unwrap(), bcs::to_bytes(&amount).unwrap()],
    ))
}

fn decode_claim_mint_capability_script_function(
    payload: &TransactionPayload,
) -> Option<ScriptFunctionCall> {
    if let TransactionPayload::ScriptFunction(_script) = payload {
        Some(ScriptFunctionCall::ClaimMintCapability {})
    } else {
        None
    }
}

fn decode_create_account_script_function(
    payload: &TransactionPayload,
) -> Option<ScriptFunctionCall> {
    if let TransactionPayload::ScriptFunction(script) = payload {
        Some(ScriptFunctionCall::CreateAccount {
            new_account_address: bcs::from_bytes(script.args().get(0)?).ok()?,
            auth_key_prefix: bcs::from_bytes(script.args().get(1)?).ok()?,
        })
    } else {
        None
    }
}

fn decode_delegate_mint_capability_script_function(
    payload: &TransactionPayload,
) -> Option<ScriptFunctionCall> {
    if let TransactionPayload::ScriptFunction(script) = payload {
        Some(ScriptFunctionCall::DelegateMintCapability {
            addr: bcs::from_bytes(script.args().get(0)?).ok()?,
        })
    } else {
        None
    }
}

fn decode_mint_script_function(payload: &TransactionPayload) -> Option<ScriptFunctionCall> {
    if let TransactionPayload::ScriptFunction(script) = payload {
        Some(ScriptFunctionCall::Mint {
            addr: bcs::from_bytes(script.args().get(0)?).ok()?,
            amount: bcs::from_bytes(script.args().get(1)?).ok()?,
        })
    } else {
        None
    }
}

fn decode_rotate_authentication_key_script_function(
    payload: &TransactionPayload,
) -> Option<ScriptFunctionCall> {
    if let TransactionPayload::ScriptFunction(script) = payload {
        Some(ScriptFunctionCall::RotateAuthenticationKey {
            new_authentication_key: bcs::from_bytes(script.args().get(0)?).ok()?,
        })
    } else {
        None
    }
}

fn decode_set_gas_constants_script_function(
    payload: &TransactionPayload,
) -> Option<ScriptFunctionCall> {
    if let TransactionPayload::ScriptFunction(script) = payload {
        Some(ScriptFunctionCall::SetGasConstants {
            global_memory_per_byte_cost: bcs::from_bytes(script.args().get(0)?).ok()?,
            global_memory_per_byte_write_cost: bcs::from_bytes(script.args().get(1)?).ok()?,
            min_transaction_gas_units: bcs::from_bytes(script.args().get(2)?).ok()?,
            large_transaction_cutoff: bcs::from_bytes(script.args().get(3)?).ok()?,
            intrinsic_gas_per_byte: bcs::from_bytes(script.args().get(4)?).ok()?,
            maximum_number_of_gas_units: bcs::from_bytes(script.args().get(5)?).ok()?,
            min_price_per_gas_unit: bcs::from_bytes(script.args().get(6)?).ok()?,
            max_price_per_gas_unit: bcs::from_bytes(script.args().get(7)?).ok()?,
            max_transaction_size_in_bytes: bcs::from_bytes(script.args().get(8)?).ok()?,
            gas_unit_scaling_factor: bcs::from_bytes(script.args().get(9)?).ok()?,
            default_account_size: bcs::from_bytes(script.args().get(10)?).ok()?,
        })
    } else {
        None
    }
}

fn decode_transfer_script_function(payload: &TransactionPayload) -> Option<ScriptFunctionCall> {
    if let TransactionPayload::ScriptFunction(script) = payload {
        Some(ScriptFunctionCall::Transfer {
            to: bcs::from_bytes(script.args().get(0)?).ok()?,
            amount: bcs::from_bytes(script.args().get(1)?).ok()?,
        })
    } else {
        None
    }
}

type ScriptFunctionDecoderMap = std::collections::HashMap<
    String,
    Box<
        dyn Fn(&TransactionPayload) -> Option<ScriptFunctionCall>
            + std::marker::Sync
            + std::marker::Send,
    >,
>;

static SCRIPT_FUNCTION_DECODER_MAP: once_cell::sync::Lazy<ScriptFunctionDecoderMap> =
    once_cell::sync::Lazy::new(|| {
        let mut map: ScriptFunctionDecoderMap = std::collections::HashMap::new();
        map.insert(
            "AdminScriptsclaim_mint_capability".to_string(),
            Box::new(decode_claim_mint_capability_script_function),
        );
        map.insert(
            "BasicScriptscreate_account".to_string(),
            Box::new(decode_create_account_script_function),
        );
        map.insert(
            "AdminScriptsdelegate_mint_capability".to_string(),
            Box::new(decode_delegate_mint_capability_script_function),
        );
        map.insert(
            "AdminScriptsmint".to_string(),
            Box::new(decode_mint_script_function),
        );
        map.insert(
            "BasicScriptsrotate_authentication_key".to_string(),
            Box::new(decode_rotate_authentication_key_script_function),
        );
        map.insert(
            "AdminScriptsset_gas_constants".to_string(),
            Box::new(decode_set_gas_constants_script_function),
        );
        map.insert(
            "BasicScriptstransfer".to_string(),
            Box::new(decode_transfer_script_function),
        );
        map
    });
