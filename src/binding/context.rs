use std::collections::HashMap;

use extism_pdk::*;

#[host_fn]
extern "ExtismHost" {
    pub fn require_safe() -> bool;
}

pub struct Context {
    pub root_claims: HashMap<String, String>,
}
