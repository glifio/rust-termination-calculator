// use fvm_shared::sector::{RegisteredSealProof, SectorNumber};
use fvm_shared::clock::ChainEpoch;
// use fvm_shared::deal::DealID;
use fvm_shared::econ::TokenAmount;
use fil_actors_runtime::DealWeight;
// use cid::Cid;
use fvm_ipld_encoding::tuple::*;
use fvm_shared::bigint::bigint_ser;
use serde::{Deserialize, Serialize};

// From filecoin-project/builtin-actors

/// Information stored on-chain for a proven sector.
#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize_tuple, Deserialize_tuple)]
pub struct SectorOnChainInfo {
    // pub sector_number: SectorNumber, // unused
    /// The seal proof type implies the PoSt proofs
    // pub seal_proof: RegisteredSealProof, // unused
    /// CommR
    // pub sealed_cid: Cid, // unused
    // pub deal_ids: Vec<DealID>, // unused
    /// Epoch during which the sector proof was accepted
    pub activation: ChainEpoch,
    /// Epoch during which the sector expires
    pub expiration: ChainEpoch,
    /// Integral of active deals over sector lifetime
    #[serde(with = "bigint_ser")]
    pub deal_weight: DealWeight,
    /// Integral of active verified deals over sector lifetime
    #[serde(with = "bigint_ser")]
    pub verified_deal_weight: DealWeight,
    /// Pledge collected to commit this sector
    // pub initial_pledge: TokenAmount, // unused
    /// Expected one day projection of reward for sector computed at activation / update / extension time
    pub expected_day_reward: TokenAmount,
    /// Expected twenty day projection of reward for sector computed at activation / update / extension time
    pub expected_storage_pledge: TokenAmount,
    /// Epoch at which this sector's power was most recently updated
    pub power_base_epoch: ChainEpoch,
    /// Maximum day reward this sector has had in previous iterations (zero for brand new sectors)
    pub replaced_day_reward: TokenAmount,
    // The original SealedSectorCID, only gets set on the first ReplicaUpdate
    // pub sector_key_cid: Option<Cid>, // unused
    // Additional flags, see [`SectorOnChainInfoFlags`]
    // pub flags: SectorOnChainInfoFlags, // unused
}

bitflags::bitflags! {
    #[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default, Debug)]
    #[serde(transparent)]
    pub struct SectorOnChainInfoFlags: u32 {
        /// QA power mechanism introduced in FIP-0045
        const SIMPLE_QA_POWER = 0x1;
    }
}
