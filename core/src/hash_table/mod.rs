pub mod actor;
pub mod entry;
pub mod file;
pub mod memory;
pub mod pair;
pub mod meta;
pub mod status;
pub mod sys_entry;
#[cfg(test)]
pub mod test_util;

use agent::keys::Keys;
use error::HolochainError;
use hash_table::{
    meta::EntryMeta,
    status::{CrudStatus, LINK_NAME, STATUS_NAME},
};
use key::Key;
use hash_table::entry::Entry;

pub type HashString = String;

/// Trait of the data structure storing the source chain
/// source chain is stored as a hash table of Pairs.
/// Pair is a pair holding an Entry and its Header
pub trait HashTable: Send + Sync + Clone + 'static {
    // internal state management
    // @TODO does this make sense at the trait level?
    // @see https://github.com/holochain/holochain-rust/issues/262
    fn setup(&mut self) -> Result<(), HolochainError> {
        Ok(())
    }
    fn teardown(&mut self) -> Result<(), HolochainError> {
        Ok(())
    }

    // crud
    /// add a Entry to the HashTable, analogous to chain.push() but ordering is not enforced
    fn put_entry(&mut self, entry: &Entry) -> Result<(), HolochainError>;

    /// lookup a Entry from the HashTable by Entry key
    fn get_entry(&self, key: &str) -> Result<Option<Entry>, HolochainError>;

    /// add a new Entry to the HashTable as per commit and status link an old Entry as MODIFIED
    fn modify_entry(
        &mut self,
        keys: &Keys,
        old_entry: &Entry,
        new_entry: &Entry,
    ) -> Result<(), HolochainError> {
        self.put_entry(new_entry)?;

        // @TODO what if meta fails when commit succeeds?
        // @see https://github.com/holochain/holochain-rust/issues/142
        self.assert_entry_meta(&EntryMeta::new(
            keys,
            &old_entry,
            STATUS_NAME,
            &CrudStatus::MODIFIED.bits().to_string(),
        ))?;

        // @TODO what if meta fails when commit succeeds?
        // @see https://github.com/holochain/holochain-rust/issues/142
        self.assert_entry_meta(&EntryMeta::new(keys, &old_entry, LINK_NAME, &new_entry.key()))
    }

    /// set the status of a Pair to DELETED
    fn retract_entry(&mut self, keys: &Keys, entry: &Entry) -> Result<(), HolochainError> {
        self.assert_entry_meta(&EntryMeta::new(
            keys,
            &entry,
            STATUS_NAME,
            &CrudStatus::DELETED.bits().to_string(),
        ))
    }

    // meta
    /// assert a given EntryMeta in the HashTable
    fn assert_entry_meta(&mut self, meta: &EntryMeta) -> Result<(), HolochainError>;

    /// lookup a EntryMeta from the HashTable by EntryMeta key
    fn entry_meta(&mut self, key: &str) -> Result<Option<EntryMeta>, HolochainError>;
    /// lookup all EntryMeta for a given Entry
    fn metas_for_entry(&mut self, entry: &Entry) -> Result<Vec<EntryMeta>, HolochainError>;

    // query
    // @TODO how should we handle queries?
    // @see https://github.com/holochain/holochain-rust/issues/141
    // fn query (&self, query: &Query) -> Result<std::collections::HashSet, HolochainError>;
}
