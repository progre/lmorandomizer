use std::{fmt::Debug, sync::Arc};

use super::{LmoDelegate, OriginalAddrs};

#[derive(Debug)]
pub struct LmoHookManager {
    pub delegate: Arc<dyn LmoDelegate>,
    pub original_addrs: OriginalAddrs,
}

impl LmoHookManager {
    pub fn new(delegate: Arc<dyn LmoDelegate>, original_addrs: OriginalAddrs) -> Self {
        Self {
            delegate,
            original_addrs,
        }
    }
}

unsafe impl Send for LmoHookManager {}
unsafe impl Sync for LmoHookManager {}
