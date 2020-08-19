use std::collections::HashMap;

use super::runtime_value::*;

struct ScopeEntry {
    original: RuntimeValue,
    shadows: Vec<RuntimeValue>,
}

pub struct ScopeStack {
    lookup: HashMap<String, ScopeEntry>,
}

impl ScopeStack {
    pub fn new() -> Self {
        Self {
            lookup: HashMap::new(),
        }
    }

    pub fn assign(&mut self, varname: &str, value: RuntimeValue) {
        match self.lookup.get_mut(varname) {
            None => {
                self.lookup.insert(
                    varname.to_owned(),
                    ScopeEntry {
                        original: value,
                        shadows: vec![],
                    },
                );
            }
            Some(entry) => {
                if let Some(shadow) = entry.shadows.last_mut() {
                    *shadow = value;
                } else {
                    entry.original = value
                }
            }
        }
    }

    pub fn resolve(&self, varname: &str) -> Option<RuntimeValue> {
        match self.lookup.get(varname) {
            None => None,
            Some(entry) => {
                if let Some(shadow) = entry.shadows.last() {
                    Some(shadow.clone())
                } else {
                    Some(entry.original.clone())
                }
            }
        }
    }

    pub fn save(&mut self, varname: &str) {
        let default_value = RuntimeValue::Boolean(false);

        match self.lookup.get_mut(varname) {
            None => {
                self.lookup.insert(
                    varname.to_owned(),
                    ScopeEntry {
                        original: default_value,
                        shadows: vec![],
                    },
                );
            }
            Some(entry) => {
                entry.shadows.push(default_value);
            }
        }
    }

    pub fn restore(&mut self, varname: &str) {
        if let Some(entry) = self.lookup.get_mut(varname) {
            if entry.shadows.is_empty() {
                self.lookup.remove(varname);
            } else {
                entry.shadows.pop();
            }
        }
    }
}
