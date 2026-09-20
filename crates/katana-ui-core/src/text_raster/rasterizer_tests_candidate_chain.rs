use super::*;

#[test]
fn candidate_chain_keeps_copied_candidates_ordered_and_wraps_with_finite_geometry()
-> Result<(), Box<dyn std::error::Error>> {
    let (source, _) = installed_font_candidate()?;
    let proportional_primary = copy_font_candidate(&source)?;
    let proportional_fallback = copy_font_candidate(&source)?;
    let monospace_primary = copy_font_candidate(&source)?;
    let monospace_fallback = copy_font_candidate(&source)?;
    let copied_paths = [
        proportional_primary.clone(),
        proportional_fallback.clone(),
        monospace_primary.clone(),
        monospace_fallback.clone(),
    ];
    let result = (|| -> Result<(), Box<dyn std::error::Error>> {
        let config = candidate_chain_config_for_faces(
            vec![proportional_primary.clone(), proportional_fallback.clone()],
            vec![monospace_primary.clone(), monospace_fallback],
        );
        let catalog = Arc::new(PlatformFontCatalog::new(config.catalog_policy()));
        let faces = catalog.regular_font_faces();
        assert_eq!(2, faces.proportional.len());
        assert_eq!(2, faces.monospace.len());
        let mut rasterizer = PlatformTextRasterizer::with_catalog_and_face_selection(
            catalog,
            config,
            PlatformTextFaceSelection::CandidateChain,
        )?;
        let selected_family = rasterizer
            .text_faces
            .proportional()
            .expect("candidate chain proportional alias")
            .to_owned();
        assert_eq!(
            candidate_chain_alias_sources(&rasterizer, &selected_family)?,
            vec![proportional_primary.clone(), proportional_fallback.clone()]
        );
        assert_eq!(
            first_shaped_font_source(
                &rasterizer,
                PlatformTextFaceSelection::CandidateChain,
                &selected_family,
            )?,
            proportional_primary
        );

        let mut request = PlatformTextRasterRequest::from_text(
            "Candidate chain must retain a finite wrapped layout.",
            font(FontFamily::Proportional),
            TEXT_COLOR,
        );
        request.max_width_px = Some(36.0);
        let raster = rasterizer.rasterize(&request)?;
        assert!(raster.width > 0 && raster.height > 0);
        assert!(raster.height as f32 > request.normalized_line_height());
        Ok(())
    })();
    for path in copied_paths {
        let _ = fs::remove_file(path);
    }
    result
}

#[test]
fn candidate_chain_skips_missing_leading_candidate_before_shaping()
-> Result<(), Box<dyn std::error::Error>> {
    let (source, _) = installed_font_candidate()?;
    let candidate = copy_font_candidate(&source)?;
    let missing = missing_font_path();
    let result = (|| -> Result<(), Box<dyn std::error::Error>> {
        let config = candidate_chain_config_for_faces(
            vec![missing.clone(), candidate.clone()],
            vec![candidate.clone()],
        );
        let catalog = Arc::new(PlatformFontCatalog::new(config.catalog_policy()));
        let mut rasterizer = PlatformTextRasterizer::with_catalog_and_face_selection(
            catalog,
            config,
            PlatformTextFaceSelection::CandidateChain,
        )?;
        let raster = rasterizer.rasterize(&PlatformTextRasterRequest::from_text(
            SOURCE_IDENTITY_TEXT,
            font(FontFamily::Proportional),
            TEXT_COLOR,
        ))?;

        assert!(raster.width > 0 && raster.height > 0);
        let family = rasterizer
            .text_faces
            .proportional()
            .expect("candidate chain must resolve a valid fallback")
            .to_owned();
        assert_eq!(
            first_shaped_font_source(
                &rasterizer,
                PlatformTextFaceSelection::CandidateChain,
                &family,
            )?,
            candidate
        );
        Ok(())
    })();
    let _ = fs::remove_file(candidate);
    result
}

#[test]
fn unresolved_candidate_selection_keeps_generic_fallback_faces()
-> Result<(), Box<dyn std::error::Error>> {
    let missing = missing_font_path();
    let config = PlatformTextRasterConfig {
        proportional_candidates: vec![missing.clone()],
        monospace_candidates: vec![missing],
        emoji_candidates: Vec::new(),
        emoji_candidate_sha256: Vec::new(),
        cache_capacity: 4,
    };
    for face_selection in [
        PlatformTextFaceSelection::FirstCandidate,
        PlatformTextFaceSelection::CandidateChain,
    ] {
        let catalog = Arc::new(PlatformFontCatalog::new(config.catalog_policy()));
        let mut rasterizer = PlatformTextRasterizer::with_catalog_and_face_selection(
            catalog,
            config.clone(),
            face_selection,
        )?;
        assert_eq!(rasterizer.text_faces, ResolvedTextFaces::default());
        let raster = rasterizer.rasterize(&PlatformTextRasterRequest::from_text(
            "System fallback",
            font(FontFamily::Proportional),
            TEXT_COLOR,
        ))?;
        assert!(raster.width > 0 && raster.height > 0);
    }
    Ok(())
}
