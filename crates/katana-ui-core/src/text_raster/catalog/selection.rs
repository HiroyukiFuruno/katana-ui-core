use super::{PlatformRegularFontFace, PlatformRegularFontFaces, catalog_cache};
use cosmic_text::{
    FontSystem,
    fontdb::{Database, FaceInfo, ID},
};

pub(super) fn first_candidate_font_system(
    locale: String,
    mut database: Database,
    selected_faces: &PlatformRegularFontFaces,
) -> FontSystem {
    let aliases = selected_faces
        .iter()
        .flat_map(|selected| {
            database
                .faces()
                .filter(move |face| face_matches_selected_candidate(face, selected))
                .cloned()
                .map(move |mut face| {
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

fn face_matches_selected_candidate(face: &FaceInfo, selected: &PlatformRegularFontFace) -> bool {
    catalog_cache::file_path_from_source(&face.source) == Some(selected.source_file_path.as_path())
        && face
            .families
            .iter()
            .any(|(family, _)| family == &selected.family)
}
