use crate::id_gen::id_generator_options::IdGeneratorOptions;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Snowflake {
    options: IdGeneratorOptions,
    sequence: i64,
    last_timestamp: i64,
    lock: Mutex<()>,
}

impl Snowflake {
    pub fn new(options: IdGeneratorOptions) -> Self {
        Snowflake {
            options,
            sequence: 0,
            last_timestamp: -1,
            lock: Mutex::new(()),
        }
    }

    fn timestamp(&self) -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as i64
    }

    fn til_next_millis(&self, last_timestamp: i64) -> i64 {
        let mut timestamp = self.timestamp();
        while timestamp <= last_timestamp {
            timestamp = self.timestamp();
        }
        timestamp
    }

    pub fn next_id(&mut self) -> Result<i64, &'static str> {
        let _lock = self.lock.lock().unwrap();

        let mut timestamp = self.timestamp();

        if timestamp < self.last_timestamp {
            return Err("Clock moved backwards. Refusing to generate id_gen");
        }

        if timestamp == self.last_timestamp {
            self.sequence = (self.sequence + 1) & ((1 << self.options.seq_bit_length) - 1);
            if self.sequence == 0 {
                timestamp = self.til_next_millis(self.last_timestamp);
            }
        } else {
            self.sequence = 0;
        }

        self.last_timestamp = timestamp;

        let id = ((timestamp - self.options.base_time)
            << (self.options.worker_id_bit_length + self.options.seq_bit_length))
            | (self.options.worker_id << self.options.seq_bit_length)
            | self.sequence;

        Ok(id)
    }
}

pub struct DefaultIdGenerator {
    snowflake: Option<Arc<Mutex<Snowflake>>>,
}

impl DefaultIdGenerator {
    pub fn new() -> Self {
        DefaultIdGenerator { snowflake: None }
    }

    pub fn set_id_generator(&mut self, options: IdGeneratorOptions) {
        self.snowflake = Some(Arc::new(Mutex::new(Snowflake::new(options))));
    }

    pub fn next_id(&self) -> Result<i64, &'static str> {
        if let Some(ref snowflake) = self.snowflake {
            let mut snowflake = snowflake.lock().unwrap();
            snowflake.next_id()
        } else {
            Err("please set id_gen generator at first.")
        }
    }
}
