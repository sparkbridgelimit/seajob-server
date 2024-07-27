use std::sync::{LazyLock, Mutex};
use crate::id_gen::default_id_generator::{DefaultIdGenerator};
use crate::id_gen::id_generator_options::IdGeneratorOptions;

pub static GLOBAL_IDGEN: LazyLock<Mutex<IDGenerator>> = LazyLock::new(|| Mutex::new(
    IDGenerator::new()
));

pub struct IDGenerator {
    idgen: DefaultIdGenerator,
}

impl IDGenerator {
    pub fn new() -> Self {
        let mut idgen = DefaultIdGenerator::new();
        let options = IdGeneratorOptions::new(82373722637, 6, 6);
        idgen.set_id_generator(options);
        IDGenerator { idgen }
    }

    pub fn next_id(&self) -> Result<i64, &'static str> {
        self.idgen.next_id()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_generator_new() {
        let idgen = IDGenerator::new();
        let id1 = idgen.next_id().unwrap();
        let id2 = idgen.next_id().unwrap();
        assert!(id2 > id1);
    }
}