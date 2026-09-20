use crate::text_raster::catalog::PlatformRegularFontFaces;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct ResolvedTextFaces {
    proportional: Option<ResolvedTextFace>,
    monospace: Option<ResolvedTextFace>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedTextFace {
    family: String,
    weight: u16,
    style: cosmic_text::Style,
    stretch: cosmic_text::Stretch,
}

impl ResolvedTextFaces {
    #[cfg(test)]
    pub(crate) fn from_first_candidates(
        proportional: Option<String>,
        monospace: Option<String>,
    ) -> Self {
        Self {
            proportional: proportional.map(ResolvedTextFace::regular),
            monospace: monospace.map(ResolvedTextFace::regular),
        }
    }

    pub(crate) fn from_candidate_faces(candidates: PlatformRegularFontFaces) -> Self {
        Self {
            proportional: candidates
                .proportional
                .into_iter()
                .next()
                .map(ResolvedTextFace::from_candidate),
            monospace: candidates
                .monospace
                .into_iter()
                .next()
                .map(ResolvedTextFace::from_candidate),
        }
    }

    #[cfg(test)]
    pub(crate) fn proportional(&self) -> Option<&str> {
        self.proportional.as_ref().map(|face| face.family.as_str())
    }

    #[cfg(test)]
    pub(crate) fn monospace(&self) -> Option<&str> {
        self.monospace.as_ref().map(|face| face.family.as_str())
    }

    pub(crate) fn selected_face(
        &self,
        family: crate::theme::FontFamily,
        style: &crate::render_model::UiTextSpanStyle,
        text: &str,
    ) -> Option<&ResolvedTextFace> {
        if style.emoji {
            return None;
        }
        if text.is_ascii() && (style.monospace || family == crate::theme::FontFamily::Monospace) {
            self.monospace.as_ref()
        } else {
            self.proportional.as_ref()
        }
    }

    pub(crate) fn proportional_face(&self) -> Option<&ResolvedTextFace> {
        self.proportional.as_ref()
    }
}

impl ResolvedTextFace {
    #[cfg(test)]
    fn regular(family: String) -> Self {
        Self {
            family,
            weight: super::REGULAR_WEIGHT,
            style: cosmic_text::Style::Normal,
            stretch: cosmic_text::Stretch::Normal,
        }
    }

    fn from_candidate(face: crate::text_raster::catalog::PlatformRegularFontFace) -> Self {
        Self {
            family: face.selection_family,
            weight: face.weight,
            style: face.style,
            stretch: face.stretch,
        }
    }

    pub(crate) fn family(&self) -> &str {
        &self.family
    }
    pub(crate) fn weight(&self) -> u16 {
        self.weight
    }
    pub(crate) fn style(&self) -> cosmic_text::Style {
        self.style
    }
    pub(crate) fn stretch(&self) -> cosmic_text::Stretch {
        self.stretch
    }
}
