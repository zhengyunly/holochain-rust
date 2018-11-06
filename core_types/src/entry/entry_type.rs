#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct AppEntryType(String);

// Enum for listing all System Entry Types
// Variant `Data` is for user defined entry types
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub enum EntryType {
    App(AppEntryType),
    Dna,
    AgentId,
    Delete,
    LinkAdd,
    LinkRemove,
    LinkList,
    ChainHeader,
    ChainMigrate,
}

impl Eq for EntryType {}

impl EntryType {

    pub fn can_publish(&self) -> bool {
        match self {
            EntryType::Dna => false,
            _ => true,
        }
    }

    pub fn is_sys(&self) -> bool {
        match self {
            EntryType::App(_) => false,
            _ => true,
        }
    }

    pub fn is_app(&self) -> bool {
        !self.is_sys()
    }

}

/// dummy entry type
#[cfg_attr(tarpaulin, skip)]
pub fn test_entry_type() -> EntryType {
    EntryType::App(String::from("testEntryType"))
}
