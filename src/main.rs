use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    epoch: String,

    #[arg(long)]
    sector_size: String,

    #[arg(long)]
    qap_position: String,

    #[arg(long)]
    qap_velocity: String,

    #[arg(long)]
    reward_position: String,

    #[arg(long)]
    reward_velocity: String,

    #[arg(long)]
    activation: String,

    #[arg(long)]
    expiration: String,

    #[arg(long)]
    deal_weight: String,

    #[arg(long)]
    verified_deal_weight: String,

    #[arg(long)]
    expected_day_reward: String,

    #[arg(long)]
    expected_storage_pledge: String,

    #[arg(long)]
    power_base_epoch: String,

    #[arg(long)]
    replaced_day_reward: String,
}

fn main() {
    let args = Args::parse();

    println!("Epoch: {}", args.epoch);
    println!("Sector Size: {}", args.sector_size);
    println!("QAP Position: {}", args.qap_position);
    println!("QAP Velocity: {}", args.qap_velocity);
    println!("Reward Position: {}", args.reward_position);
    println!("Reward Velocity: {}", args.reward_velocity);
    println!("Activation: {}", args.activation);
    println!("Expiration: {}", args.expiration);
    println!("Deal Weight: {}", args.deal_weight);
    println!("Verified Deal Weight: {}", args.verified_deal_weight);
    println!("Expected Day Reward: {}", args.expected_day_reward);
    println!("Expected Storage Pledge: {}", args.expected_storage_pledge);
    println!("Power Base Epoch: {}", args.power_base_epoch);
    println!("Replaced Day Reward: {}", args.replaced_day_reward);
}
