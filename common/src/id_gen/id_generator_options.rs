#[derive(Debug)]
pub struct IdGeneratorOptions {
    pub method: i8,
    pub base_time: i64,
    pub worker_id: i64,
    pub worker_id_bit_length: i8,
    pub seq_bit_length: i8,
    pub max_seq_number: i64,
    pub min_seq_number: i64,
    pub top_over_cost_count: i64,
}

impl IdGeneratorOptions {
    pub fn new(worker_id: i64, worker_id_bit_length: i8, seq_bit_length: i8) -> Self {
        IdGeneratorOptions {
            method: 1,
            base_time: 1582136402000,
            worker_id,
            worker_id_bit_length,
            seq_bit_length,
            max_seq_number: 0,
            min_seq_number: 5,
            top_over_cost_count: 2000,
        }
    }
}
