#[path = "legacy_dod_specs_atoms.rs"]
mod legacy_dod_specs_atoms;
#[path = "legacy_dod_specs_molecules.rs"]
mod legacy_dod_specs_molecules;

const PRESET_MARKER_COUNT: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct LegacyDodSpec {
    pub(super) page: &'static str,
    pub(super) marker: &'static str,
    pub(super) option: &'static str,
    pub(super) value_type: &'static str,
    pub(super) after: &'static str,
    pub(super) presets: [&'static str; PRESET_MARKER_COUNT],
}

pub(super) fn legacy_dod_specs() -> impl Iterator<Item = &'static LegacyDodSpec> {
    legacy_dod_specs_atoms::ATOM_SPECS
        .iter()
        .chain(legacy_dod_specs_molecules::MOLECULE_SPECS.iter())
}

pub(super) const fn spec(
    page: &'static str,
    marker: &'static str,
    option: &'static str,
    value_type: &'static str,
    after: &'static str,
    presets: [&'static str; PRESET_MARKER_COUNT],
) -> LegacyDodSpec {
    LegacyDodSpec {
        page,
        marker,
        option,
        value_type,
        after,
        presets,
    }
}
