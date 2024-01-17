use std::cmp;

// use cid::Cid;
use fil_actors_runtime::DealWeight;
use fvm_shared::bigint::{BigInt, Integer};
use fvm_shared::clock::ChainEpoch;
use fvm_shared::clock::EPOCH_DURATION_SECONDS;
use fvm_shared::econ::TokenAmount;
use fvm_shared::math::PRECISION;
use fvm_shared::sector::StoragePower;
// use fvm_shared::sector::{RegisteredSealProof, SectorNumber, SectorQuality, SectorSize};
use fvm_shared::sector::{SectorQuality, SectorSize};
use fvm_shared::smooth::{self, FilterEstimate};
use lazy_static::lazy_static;
use num_traits::Zero;

mod types;

pub fn terminate_sectors(
    epoch: i64,
    sector_size: SectorSize,
    qap_position: BigInt,
    qap_velocity: BigInt,
    reward_position: BigInt,
    reward_velocity: BigInt,
    activation: i64,
    expiration: i64,
    deal_weight: BigInt,
    verified_deal_weight: BigInt,
    expected_day_reward: BigInt,
    expected_storage_pledge: BigInt,
    power_base_epoch: i64,
    replaced_day_reward: BigInt,
) -> TokenAmount {
    let network_qa_power_estimate = &FilterEstimate {
        position: qap_position,
        velocity: qap_velocity,
    };

    let reward_estimate = &FilterEstimate {
        position: reward_position,
        velocity: reward_velocity,
    };

    let sector = &types::SectorOnChainInfo {
        // sector_number: SectorNumber::default(),
        // seal_proof: RegisteredSealProof::StackedDRG32GiBV1P1,
        // sealed_cid: Cid::default(),
        // deal_ids: Vec::new(),
        activation: activation,
        expiration: expiration,
        deal_weight: deal_weight,
        verified_deal_weight: verified_deal_weight,
        // initial_pledge: TokenAmount::zero(),
        expected_day_reward: TokenAmount::from_atto(expected_day_reward),
        expected_storage_pledge: TokenAmount::from_atto(expected_storage_pledge),
        power_base_epoch: power_base_epoch,
        replaced_day_reward: TokenAmount::from_atto(replaced_day_reward),
        // sector_key_cid: None,
        // flags: types::SectorOnChainInfoFlags::empty(),
    };

    let current_epoch = epoch;

    let sector_power = qa_power_for_sector(sector_size, sector);

    return pledge_penalty_for_termination(
        &sector.expected_day_reward,
        current_epoch - sector.power_base_epoch,
        &sector.expected_storage_pledge,
        network_qa_power_estimate,
        &sector_power,
        reward_estimate,
        &sector.replaced_day_reward,
        sector.power_base_epoch - sector.activation,
    );
}

// From filecoin-project/builtin-actors

pub const SECONDS_IN_HOUR: i64 = 3600;
pub const SECONDS_IN_DAY: i64 = 86400;
pub const SECONDS_IN_YEAR: i64 = 31556925;
pub const EPOCHS_IN_HOUR: i64 = SECONDS_IN_HOUR / EPOCH_DURATION_SECONDS;
pub const EPOCHS_IN_DAY: i64 = SECONDS_IN_DAY / EPOCH_DURATION_SECONDS;
pub const EPOCHS_IN_YEAR: i64 = SECONDS_IN_YEAR / EPOCH_DURATION_SECONDS;

// Maximum number of lifetime days penalized when a sector is terminated.
pub const TERMINATION_LIFETIME_CAP: ChainEpoch = 140;

const TERMINATION_PENALTY_LOWER_BOUND_PROJECTIONS_PERIOD: ChainEpoch = (EPOCHS_IN_DAY * 35) / 10;

pub const TERMINATION_REWARD_FACTOR_NUM: u32 = 1;
pub const TERMINATION_REWARD_FACTOR_DENOM: u32 = 2;

/// Precision used for making QA power calculations
pub const SECTOR_QUALITY_PRECISION: i64 = 20;

lazy_static! {
    /// Quality multiplier for committed capacity (no deals) in a sector
    pub static ref QUALITY_BASE_MULTIPLIER: BigInt = BigInt::from(10);

    /// Quality multiplier for unverified deals in a sector
    pub static ref DEAL_WEIGHT_MULTIPLIER: BigInt = BigInt::from(10);

    /// Quality multiplier for verified deals in a sector
    pub static ref VERIFIED_DEAL_WEIGHT_MULTIPLIER: BigInt = BigInt::from(100);
}

/// DealWeight and VerifiedDealWeight are spacetime occupied by regular deals and verified deals in a sector.
/// Sum of DealWeight and VerifiedDealWeight should be less than or equal to total SpaceTime of a sector.
/// Sectors full of VerifiedDeals will have a SectorQuality of VerifiedDealWeightMultiplier/QualityBaseMultiplier.
/// Sectors full of Deals will have a SectorQuality of DealWeightMultiplier/QualityBaseMultiplier.
/// Sectors with neither will have a SectorQuality of QualityBaseMultiplier/QualityBaseMultiplier.
/// SectorQuality of a sector is a weighted average of multipliers based on their proportions.
pub fn quality_for_weight(
    size: SectorSize,
    duration: ChainEpoch,
    deal_weight: &DealWeight,
    verified_weight: &DealWeight,
) -> SectorQuality {
    let sector_space_time = BigInt::from(size as u64) * BigInt::from(duration);
    let total_deal_space_time = deal_weight + verified_weight;

    let weighted_base_space_time =
        (&sector_space_time - total_deal_space_time) * &*QUALITY_BASE_MULTIPLIER;
    let weighted_deal_space_time = deal_weight * &*DEAL_WEIGHT_MULTIPLIER;
    let weighted_verified_space_time = verified_weight * &*VERIFIED_DEAL_WEIGHT_MULTIPLIER;
    let weighted_sum_space_time =
        weighted_base_space_time + weighted_deal_space_time + weighted_verified_space_time;
    let scaled_up_weighted_sum_space_time: SectorQuality =
        weighted_sum_space_time << SECTOR_QUALITY_PRECISION;

    scaled_up_weighted_sum_space_time
        .div_floor(&sector_space_time)
        .div_floor(&QUALITY_BASE_MULTIPLIER)
}

/// Returns the power for a sector size and weight.
pub fn qa_power_for_weight(
    size: SectorSize,
    duration: ChainEpoch,
    deal_weight: &DealWeight,
    verified_weight: &DealWeight,
) -> StoragePower {
    let quality = quality_for_weight(size, duration, deal_weight, verified_weight);
    (BigInt::from(size as u64) * quality) >> SECTOR_QUALITY_PRECISION
}

/// Returns the quality-adjusted power for a sector.
pub fn qa_power_for_sector(size: SectorSize, sector: &types::SectorOnChainInfo) -> StoragePower {
    let duration = sector.expiration - sector.power_base_epoch;
    qa_power_for_weight(
        size,
        duration,
        &sector.deal_weight,
        &sector.verified_deal_weight,
    )
}

/// Penalty to locked pledge collateral for the termination of a sector before scheduled expiry.
/// SectorAge is the time between the sector's activation and termination.
#[allow(clippy::too_many_arguments)]
pub fn pledge_penalty_for_termination(
    day_reward: &TokenAmount,
    sector_age: ChainEpoch,
    twenty_day_reward_at_activation: &TokenAmount,
    network_qa_power_estimate: &FilterEstimate,
    qa_sector_power: &StoragePower,
    reward_estimate: &FilterEstimate,
    replaced_day_reward: &TokenAmount,
    replaced_sector_age: ChainEpoch,
) -> TokenAmount {
    // max(SP(t), BR(StartEpoch, 20d) + BR(StartEpoch, 1d) * terminationRewardFactor * min(SectorAgeInDays, 140))
    // and sectorAgeInDays = sectorAge / EpochsInDay
    let lifetime_cap = TERMINATION_LIFETIME_CAP * EPOCHS_IN_DAY;
    let capped_sector_age = std::cmp::min(sector_age, lifetime_cap);

    let mut expected_reward: TokenAmount = day_reward * capped_sector_age;

    let relevant_replaced_age =
        std::cmp::min(replaced_sector_age, lifetime_cap - capped_sector_age);

    expected_reward += replaced_day_reward * relevant_replaced_age;

    let penalized_reward = expected_reward * TERMINATION_REWARD_FACTOR_NUM;
    let penalized_reward = penalized_reward.div_floor(TERMINATION_REWARD_FACTOR_DENOM);

    cmp::max(
        pledge_penalty_for_termination_lower_bound(
            reward_estimate,
            network_qa_power_estimate,
            qa_sector_power,
        ),
        twenty_day_reward_at_activation + (penalized_reward.div_floor(EPOCHS_IN_DAY)),
    )
}

/// This is the SP(t) penalty for a newly faulty sector that has not been declared.
/// SP(t) = UndeclaredFaultFactor * BR(t)
pub fn pledge_penalty_for_termination_lower_bound(
    reward_estimate: &FilterEstimate,
    network_qa_power_estimate: &FilterEstimate,
    qa_sector_power: &StoragePower,
) -> TokenAmount {
    expected_reward_for_power(
        reward_estimate,
        network_qa_power_estimate,
        qa_sector_power,
        TERMINATION_PENALTY_LOWER_BOUND_PROJECTIONS_PERIOD,
    )
}

/// The projected block reward a sector would earn over some period.
/// Also known as "BR(t)".
/// BR(t) = ProjectedRewardFraction(t) * SectorQualityAdjustedPower
/// ProjectedRewardFraction(t) is the sum of estimated reward over estimated total power
/// over all epochs in the projection period [t t+projectionDuration]
pub fn expected_reward_for_power(
    reward_estimate: &FilterEstimate,
    network_qa_power_estimate: &FilterEstimate,
    qa_sector_power: &StoragePower,
    projection_duration: ChainEpoch,
) -> TokenAmount {
    let network_qa_power_smoothed = network_qa_power_estimate.estimate();

    if network_qa_power_smoothed.is_zero() {
        return TokenAmount::from_atto(reward_estimate.estimate());
    }

    let expected_reward_for_proving_period = smooth::extrapolated_cum_sum_of_ratio(
        projection_duration,
        0,
        reward_estimate,
        network_qa_power_estimate,
    );
    let br128 = qa_sector_power * expected_reward_for_proving_period; // Q.0 * Q.128 => Q.128
    TokenAmount::from_atto(std::cmp::max(br128 >> PRECISION, Default::default()))
}
