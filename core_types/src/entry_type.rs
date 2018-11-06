use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

/// dummy entry type, same as test_type()
#[cfg_attr(tarpaulin, skip)]
pub fn test_entry_type_a() -> EntryType {
    test_entry_type()
}

/// dummy entry type, differs from test_type()
#[cfg_attr(tarpaulin, skip)]
pub fn test_entry_type_b() -> EntryType {
    EntryType::App(String::from("testEntryTypeB"))
}

#[cfg_attr(tarpaulin, skip)]
pub fn test_sys_entry_type() -> EntryType {
    EntryType::AgentId
}

#[cfg_attr(tarpaulin, skip)]
pub fn test_unpublishable_entry_type() -> EntryType {
    EntryType::Dna
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn test_types() -> Vec<EntryType> {
        vec![
            EntryType::AgentId,
            EntryType::Deletion,
            EntryType::App(String::from("foo")),
            EntryType::Dna,
            EntryType::ChainHeader,
            EntryType::Key,
            EntryType::Link,
            EntryType::Migration,
            EntryType::LinkList,
        ]
    }

    #[test]
    fn entry_type_kind() {
        assert!(EntryType::App(String::new()).is_app());
        assert!(!EntryType::App(String::new()).is_sys());
        assert!(EntryType::AgentId.is_sys());
        assert!(!EntryType::AgentId.is_app());
    }

    #[test]
    fn entry_type_valid_app_name() {
        assert!(EntryType::has_valid_app_name("agent_id"));
        assert!(!EntryType::has_valid_app_name("%agent_id"));
        assert!(!EntryType::has_valid_app_name(&String::from(
            EntryType::AgentId
        )));
        assert!(!EntryType::has_valid_app_name(&String::new()));
        assert!(EntryType::has_valid_app_name("toto"));
        assert!(!EntryType::has_valid_app_name("%%"));
        // TODO #445 - do a real regex test in has_valid_app_name()
        // assert!(EntryType::has_valid_app_name("\n"));
    }

    #[test]
    fn entry_type_as_str() {
        for (type_str, variant) in vec![
            (sys_prefix!("agent_id"), EntryType::AgentId),
            (sys_prefix!("deletion"), EntryType::Deletion),
            (sys_prefix!("dna"), EntryType::Dna),
            (sys_prefix!("chain_header"), EntryType::ChainHeader),
            (sys_prefix!("key"), EntryType::Key),
            (sys_prefix!("link"), EntryType::Link),
            (sys_prefix!("migration"), EntryType::Migration),
        ] {
            assert_eq!(
                variant,
                EntryType::from_str(type_str).expect("could not convert str to EntryType")
            );

            assert_eq!(type_str, &String::from(variant),);
        }
    }

    #[test]
    fn can_publish_test() {
        for t in test_types() {
            match t {
                EntryType::Dna => assert!(!t.can_publish()),
                _ => assert!(t.can_publish()),
            }
        }
    }
}
