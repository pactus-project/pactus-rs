use minicbor::{Decode, Encode};

#[derive(Encode, Decode)]
struct Params {
    #[n(1)]
    block_version: i32,
    #[n(2)]
    block_time_in_second: i32,
    #[n(3)]
    committee_size: i32,
    #[n(4)]
    block_reward: i64,
    #[n(5)]
    transaction_to_live_interval: i32,
    #[n(6)]
    unbond_interval: i32,
    #[n(7)]
    maximum_transaction_per_block: i32,
    #[n(8)]
    maximum_memo_length: i32,
    #[n(9)]
    fee_fraction: f64,
    #[n(10)]
    minimum_fee: i64,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            block_version: 1,
            block_time_in_second: 10,
            committee_size: 21,
            block_reward: 100000000,
            transaction_to_live_interval: 8640, // one days
            unbond_interval: 181440,            // 21 days
            maximum_transaction_per_block: 1000,
            maximum_memo_length: 1024,
            fee_fraction: 0.001,
            minimum_fee: 1000,
        }
    }
}
