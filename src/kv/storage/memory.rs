use super::{Range, Storage};
use crate::Error;
use std::collections::BTreeMap;
use std::ops::RangeBounds;

/// In-memory key-value storage. Primarily used for prototyping and testing.
#[derive(Clone, Debug)]
pub struct Memory {
    data: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl Memory {
    /// Creates a new Memory key-value storage engine
    pub fn new() -> Self {
        Self { data: BTreeMap::new() }
    }
}

impl Storage for Memory {
    fn read(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        Ok(self.data.get(key).cloned())
    }

    fn remove(&mut self, key: &[u8]) -> Result<(), Error> {
        self.data.remove(key);
        Ok(())
    }

    fn scan(&self, range: impl RangeBounds<Vec<u8>>) -> Range {
        // FIXME This copies everything into a separate vec to not have to deal with
        // lifetimes, which is pretty terrible.
        Box::new(
            self.data
                .range(range)
                .map(|(k, v)| Ok((k.clone(), v.clone())))
                .collect::<Vec<Result<_, Error>>>()
                .into_iter(),
        )
    }

    fn write(&mut self, key: &[u8], value: Vec<u8>) -> Result<(), Error> {
        self.data.insert(key.to_vec(), value);
        Ok(())
    }
}

#[cfg(test)]
impl super::TestSuite<Memory> for Memory {
    fn setup() -> Result<Self, Error> {
        Ok(Memory::new())
    }
}

#[test]
fn tests() -> Result<(), Error> {
    use super::TestSuite;
    Memory::test()
}
