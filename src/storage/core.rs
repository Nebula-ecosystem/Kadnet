use cryptal::primitives::U256;

pub(crate) struct LocalStorage {}

impl LocalStorage {
    pub(crate) fn new() -> Self {
        LocalStorage {}
    }
    pub(crate) fn contains(&self, _key: U256) -> Option<Vec<u8>> {
        unimplemented!()
    }
}
