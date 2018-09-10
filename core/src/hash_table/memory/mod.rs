use std::collections::HashMap;

use error::HolochainError;
use hash_table::{meta::EntryMeta, HashTable};
use key::Key;
use hash_table::entry::Entry;

/// Struct implementing the HashTable Trait by storing the HashTable in memory
#[derive(Serialize, Debug, Clone, PartialEq, Default)]
pub struct MemTable {
    entries: HashMap<String, Entry>,
    meta: HashMap<String, EntryMeta>,
}

impl MemTable {
    pub fn new() -> MemTable {
        MemTable {
            entries: HashMap::new(),
            meta: HashMap::new(),
        }
    }
}

impl HashTable for MemTable {
    fn put_entry(&mut self, entry: &Entry) -> Result<(), HolochainError> {
        self.entries.insert(entry.key(), entry.clone());
        Ok(())
    }

    fn get_entry(&self, key: &str) -> Result<Option<Entry>, HolochainError> {
        Ok(self.entries.get(key).cloned())
    }

    fn assert_entry_meta(&mut self, meta: &EntryMeta) -> Result<(), HolochainError> {
        self.meta.insert(meta.key(), meta.clone());
        Ok(())
    }

    fn entry_meta(&mut self, key: &str) -> Result<Option<EntryMeta>, HolochainError> {
        Ok(self.meta.get(key).cloned())
    }

    fn metas_for_entry(&mut self, entry: &Entry) -> Result<Vec<EntryMeta>, HolochainError> {
        let mut metas = self
            .meta
            .values()
            .filter(|&m| m.entry_hash() == entry.key())
            .cloned()
            .collect::<Vec<EntryMeta>>();
        // @TODO should this be sorted at all at this point?
        // @see https://github.com/holochain/holochain-rust/issues/144
        metas.sort();
        Ok(metas)
    }
}

#[cfg(test)]
pub mod tests {

    use hash_table::{memory::MemTable, test_util::standard_suite};

    pub fn test_table() -> MemTable {
        MemTable::new()
    }

    #[test]
    /// smoke test
    fn new() {
        test_table();
    }

    #[test]
    fn test_standard_suite() {
        standard_suite(&mut test_table());
    }

}
