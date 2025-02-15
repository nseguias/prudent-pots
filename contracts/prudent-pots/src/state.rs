use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GameConfig {
    pub fee: u64,
    pub fee_address: Addr,
    pub fee_reallocation: u64,
    pub game_denom: String,
    pub game_duration: u64,
    pub game_extend: u64,
    pub min_bid: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GameState {
    pub start_time: u64,
    pub end_time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenAllocation {
    pub pot_id: u8,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PlayerAllocations {
    pub allocations: Vec<TokenAllocation>,
}

pub const GAME_CONFIG: Item<GameConfig> = Item::new("game_config");
pub const GAME_STATE: Item<GameState> = Item::new("game_state");
pub const POT_STATES: Map<u8, TokenAllocation> = Map::new("pot_states");
pub const PLAYER_ALLOCATIONS: Map<Addr, PlayerAllocations> = Map::new("player_allocations");
pub const REALLOCATION_FEE_POOL: Item<Uint128> = Item::new("reallocation_fee_pool");
