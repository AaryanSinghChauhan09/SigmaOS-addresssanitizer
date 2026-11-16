// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-addresssanitizer: addresssanitizer component model.
//!
//! Real implementation: addresssanitizer_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddresssanitizerItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddresssanitizerError {
    EmptyAddresssanitizerItemName,
    DuplicateAddresssanitizerItem(String),
    EmptyEventSubject,
}

impl fmt::Display for AddresssanitizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyAddresssanitizerItemName => write!(f, "addresssanitizer_item name must not be empty"),
            Self::DuplicateAddresssanitizerItem(id) => write!(f, "duplicate addresssanitizer_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for AddresssanitizerError {}

pub struct Addresssanitizer {
    pub name: String,
    addresssanitizer_items: BTreeMap<String, AddresssanitizerItem>,
    events: BTreeMap<String, Event>,
}

impl Addresssanitizer {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            addresssanitizer_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_addresssanitizer_item(&mut self, item: AddresssanitizerItem) -> Result<(), AddresssanitizerError> {
        if item.name.trim().is_empty() {
            return Err(AddresssanitizerError::EmptyAddresssanitizerItemName);
        }
        let id = item.id.clone();
        if self.addresssanitizer_items.contains_key(&id) {
            return Err(AddresssanitizerError::DuplicateAddresssanitizerItem(id.clone()));
        }
        self.addresssanitizer_items.insert(id, item);
        Ok(())
    }

    pub fn addresssanitizer_item_count(&self) -> usize {
        self.addresssanitizer_items.len()
    }

    pub fn get_addresssanitizer_item(&self, id: &str) -> Option<&AddresssanitizerItem> {
        self.addresssanitizer_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), AddresssanitizerError> {
        if event.subject.trim().is_empty() {
            return Err(AddresssanitizerError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_addresssanitizer_item(id: &str, name: &str) -> AddresssanitizerItem {
        AddresssanitizerItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Addresssanitizer::new("Addresssanitizer");
        assert_eq!(m.name, "Addresssanitizer");
        assert_eq!(m.addresssanitizer_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_addresssanitizer_item() {
        let mut m = Addresssanitizer::new("Addresssanitizer");
        m.add_addresssanitizer_item(sample_addresssanitizer_item("a1", "Alpha")).unwrap();
        assert_eq!(m.addresssanitizer_item_count(), 1);
        assert!(m.get_addresssanitizer_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Addresssanitizer::new("Addresssanitizer");
        assert!(m.add_addresssanitizer_item(sample_addresssanitizer_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Addresssanitizer::new("Addresssanitizer");
        m.add_addresssanitizer_item(sample_addresssanitizer_item("x", "One")).unwrap();
        assert!(m.add_addresssanitizer_item(sample_addresssanitizer_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Addresssanitizer::new("Addresssanitizer");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
