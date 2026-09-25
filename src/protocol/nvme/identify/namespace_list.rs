//! Identify namespace list result parsing.

use crate::protocol::nvme::PAGE_SIZE;

/// Identify namespace list data structure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NamespaceList {
    /// Namespace identifiers, in increasing order.
    pub ids: Box<[u32]>,
}

impl NamespaceList {
    /// Size in bytes.
    pub(crate) const SIZE: usize = PAGE_SIZE;

    /// List holds every remaining identifier.
    pub(crate) fn is_last(&self) -> bool {
        const ID_COUNT: usize = PAGE_SIZE / size_of::<u32>();

        self.ids.len() < ID_COUNT
    }
}

impl From<&[u8; NamespaceList::SIZE]> for NamespaceList {
    fn from(data: &[u8; NamespaceList::SIZE]) -> Self {
        const DWORD_SIZE: usize = size_of::<u32>();

        // Identifiers in sequential order, terminated by zero entries.
        let ids = data
            .as_chunks::<DWORD_SIZE>()
            .0
            .iter()
            .map(|&x| u32::from_le_bytes(x))
            .take_while(|&x| x != 0)
            .collect();

        Self { ids }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data;

    #[test]
    fn namespace_list_parse() {
        let list = NamespaceList::from(test_data::patriot_p300::IDENTIFY_NAMESPACE_LIST);
        assert_eq!(*list.ids, [1]);
        assert!(list.is_last());
    }

    #[test]
    fn namespace_list_empty() {
        let list = NamespaceList::from(&[0; NamespaceList::SIZE]);
        assert!(list.ids.is_empty());
        assert!(list.is_last());
    }
}
