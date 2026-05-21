mod legacy_01_12;
mod legacy_13_24;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct LegacyPageContract {
    pub(super) number: u8,
    pub(super) label: &'static str,
    pub(super) page: &'static str,
    pub(super) action: &'static str,
    pub(super) event: &'static str,
    pub(super) option: &'static str,
    pub(super) after: &'static str,
    pub(super) state: &'static str,
    pub(super) preset: &'static str,
}

pub(super) fn legacy_01_24_contracts() -> impl Iterator<Item = &'static LegacyPageContract> {
    legacy_01_12::CONTRACTS
        .iter()
        .chain(legacy_13_24::CONTRACTS.iter())
}
