use super::{PlatformRegularFontFace, PlatformRegularFontFaces, catalog_cache};
use cosmic_text::{
    FontSystem,
    fontdb::{Database, FaceInfo, ID},
};

pub(super) fn selected_candidate_font_system(
    locale: String,
    mut database: Database,
    selected_faces: &PlatformRegularFontFaces,
    include_candidate_chain: bool,
) -> FontSystem {
    let selected_faces = if include_candidate_chain {
        selected_faces.iter().collect::<Vec<_>>()
    } else {
        selected_faces.first_candidates().collect::<Vec<_>>()
    };
    let aliases = selected_faces
        .into_iter()
        .filter_map(|selected| {
            database
                .faces()
                .find(|face| face_matches_selected_candidate(face, selected))
                .cloned()
                .map(|mut face| {
                    face.id = ID::dummy();
                    face.families = face
                        .families
                        .into_iter()
                        .map(|(_, language)| (selected.selection_family.clone(), language))
                        .collect();
                    face
                })
        })
        .collect::<Vec<_>>();
    for alias in aliases {
        database.push_face_info(alias);
    }
    FontSystem::new_with_locale_and_db(locale, database)
}

pub(super) fn face_matches_selected_candidate(
    face: &FaceInfo,
    selected: &PlatformRegularFontFace,
) -> bool {
    catalog_cache::file_path_from_source(&face.source) == Some(selected.source_file_path.as_path())
        && face.index == selected.index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selected_candidate_alias_requires_the_same_source_face_index() {
        let font_system = FontSystem::new();
        let face = font_system
            .db()
            .faces()
            .find(|face| catalog_cache::file_path_from_source(&face.source).is_some())
            .expect("file-backed system font");
        let selected = PlatformRegularFontFace {
            family: face.families[0].0.clone(),
            source_file_path: catalog_cache::file_path_from_source(&face.source)
                .expect("source path")
                .to_path_buf(),
            index: face.index,
            weight: face.weight.0,
            style: face.style,
            stretch: face.stretch,
            selection_family: "__candidate__".to_owned(),
        };
        let mut other_index = face.clone();
        other_index.index = other_index.index.saturating_add(1);

        assert!(face_matches_selected_candidate(face, &selected));
        assert!(!face_matches_selected_candidate(&other_index, &selected));
    }
}
