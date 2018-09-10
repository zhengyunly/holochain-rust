use agent::keys::Keys;
use error::HolochainError;
use futures::executor::block_on;
use hash_table::{pair::Pair, meta::EntryMeta};
use riker::actors::*;
use riker_default::DefaultModel;
use riker_patterns::ask::ask;
use hash_table::entry::Entry;
use hash_table::HashString;

#[derive(Clone, Debug)]
/// riker protocol for all our actors
/// currently this is flat but may be nested/namespaced in the future or multi-protocol riker
/// @see https://github.com/riker-rs/riker/issues/17
pub enum Protocol {
    /// Chain::set_top_pair()
    SetTopPair(Option<Pair>),
    SetTopPairResult(Result<Option<Pair>, HolochainError>),

    /// Chain::top_pair()
    GetTopPair,
    GetTopPairResult(Option<Pair>),

    /// HashTable::setup()
    Setup,
    SetupResult(Result<(), HolochainError>),

    /// HashTable::teardown()
    Teardown,
    TeardownResult(Result<(), HolochainError>),

    /// HashTable::get_entry()
    GetEntry(HashString),
    GetEntryResult(Result<Option<Entry>, HolochainError>),

    /// HashTable::put_entry()
    PutEntry(Entry),
    PutEntryResult(Result<(), HolochainError>),

    /// HashTable::modify_entry()
    ModifyEntry {
        keys: Keys,
        old_entry: Entry,
        new_entry: Entry,
    },
    ModifyEntryResult(Result<(), HolochainError>),

    /// HashTable::retract_entry()
    RetractEntry {
        keys: Keys,
        entry: Entry,
    },
    RetractEntryResult(Result<(), HolochainError>),

    /// HashTable::assert_entry_meta()
    AssertEntryMeta(EntryMeta),
    AssertEntryMetaResult(Result<(), HolochainError>),

    /// HashTable::entry_meta()
    GetEntryMeta(String),
    GetEntryMetaResult(Result<Option<EntryMeta>, HolochainError>),

    /// HashTable::metas_for_entry()
    GetMetasForEntry(Pair),
    GetMetasForEntryResult(Result<Vec<EntryMeta>, HolochainError>),

    /// HashTable::pair()
    GetPair(String),
    GetPairResult(Result<Option<Pair>, HolochainError>),

    /// HashTable::put_pair()
    PutPair(Pair),
    PutPairResult(Result<(), HolochainError>),
}

/// this is the global state that manages every actor
/// to be thread/concurrency safe there must only ever be one actor system
/// @see https://github.com/riker-rs/riker/issues/17
/// @see http://riker.rs/actors/#creating-actors
lazy_static! {
    pub static ref SYS: ActorSystem<Protocol> = {
        let model: DefaultModel<Protocol> = DefaultModel::new();
        ActorSystem::new(&model).unwrap()
    };
}

/// required by riker
impl Into<ActorMsg<Protocol>> for Protocol {
    fn into(self) -> ActorMsg<Protocol> {
        ActorMsg::User(self)
    }
}

/// convenience trait to build fake synchronous facades for actors
pub trait AskSelf {
    /// adapter for synchronous code to interact with an actor
    /// uses the ask() fn from riker patterns under the hood to create a future then block on it
    /// handles passing the actor system through to ask() to hide that implementation detail
    /// @see http://riker.rs/patterns/#ask
    fn block_on_ask(&self, message: Protocol) -> Protocol;
}

impl AskSelf for ActorRef<Protocol> {
    fn block_on_ask(&self, message: Protocol) -> Protocol {
        let a = ask(&(*SYS), self, message);
        block_on(a).unwrap()
    }
}
