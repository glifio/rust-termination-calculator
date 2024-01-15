use clap::Parser;
use rust_termination_calculator::terminate_sectors;
use num_bigint::BigInt;
use fvm_shared::sector::SectorSize;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    epoch: i64,

    #[arg(long)]
    sector_size: i64,

    #[arg(long)]
    qap_position: BigInt,

    #[arg(long)]
    qap_velocity: BigInt,

    #[arg(long)]
    reward_position: BigInt,

    #[arg(long)]
    reward_velocity: BigInt,

    #[arg(long)]
    activation: i64,

    #[arg(long)]
    expiration: i64,

    #[arg(long)]
    deal_weight: BigInt,

    #[arg(long)]
    verified_deal_weight: BigInt,

    #[arg(long)]
    expected_day_reward: BigInt,

    #[arg(long)]
    expected_storage_pledge: BigInt,

    #[arg(long)]
    power_base_epoch: i64,

    #[arg(long)]
    replaced_day_reward: BigInt,
}

fn main() {
    let args = Args::parse();

    let sector_size = match args.sector_size {
        32 => SectorSize::_32GiB,
        64 => SectorSize::_64GiB,
        _ => panic!("Unknown sector size")
    };

    let fee = terminate_sectors(
        args.epoch,
        sector_size,
        args.qap_position,
        args.qap_velocity,
        args.reward_position,
        args.reward_velocity,
        args.activation,
        args.expiration,
        args.deal_weight,
        args.verified_deal_weight,
        args.expected_day_reward,
        args.expected_storage_pledge,
        args.power_base_epoch,
        args.replaced_day_reward
    );
    println!("{}", fee.atto());
}
