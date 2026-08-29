use super::super::types::{EguiTextCommandSurfacePresentation, TextCommandSurfaceStyle};
use super::EguiTextCommandSurfaceHostTargetToken;
use super::{
    EguiTextCommandSurfaceCommandFamilyProjection, EguiTextCommandSurfacePresentationToken,
    EguiTextCommandSurfaceRootFactoryError, RootPresentationWire,
    RootPresentationWireWithCommandFamilies,
};
use sha2::{Digest, Sha256};

pub(super) struct DecodedRootPresentation {
    pub(super) identity: String,
    pub(super) presentation: EguiTextCommandSurfacePresentation,
    pub(super) style: TextCommandSurfaceStyle,
    pub(super) command_families: EguiTextCommandSurfaceCommandFamilyProjection,
}

const COMMAND_FAMILY_WIRE_VERSION: u8 = 1;

pub(super) fn decode_token(
    token: &EguiTextCommandSurfacePresentationToken,
) -> Result<DecodedRootPresentation, EguiTextCommandSurfaceRootFactoryError> {
    if token.target.payload.is_empty() {
        return Err(EguiTextCommandSurfaceRootFactoryError::InvalidToken(
            "host target is empty",
        ));
    }
    let identity = format!("{:x}", Sha256::digest(&token.target.payload));
    let value = serde_json::from_slice::<serde_json::Value>(&token.payload)
        .map_err(|error| EguiTextCommandSurfaceRootFactoryError::Decode(error.to_string()))?;
    if value.get("version").is_some() {
        let wire = serde_json::from_value::<RootPresentationWireWithCommandFamilies>(value)
            .map_err(|error| EguiTextCommandSurfaceRootFactoryError::Decode(error.to_string()))?;
        if wire.version != COMMAND_FAMILY_WIRE_VERSION {
            return Err(EguiTextCommandSurfaceRootFactoryError::Decode(
                "unsupported root presentation token version".to_owned(),
            ));
        }
        return Ok(DecodedRootPresentation {
            identity,
            presentation: wire.presentation,
            style: wire.style,
            command_families: wire.command_families,
        });
    }
    let wire = serde_json::from_value::<RootPresentationWire>(value)
        .map_err(|error| EguiTextCommandSurfaceRootFactoryError::Decode(error.to_string()))?;
    Ok(DecodedRootPresentation {
        identity,
        presentation: wire.presentation,
        style: wire.style,
        command_families: EguiTextCommandSurfaceCommandFamilyProjection::legacy_compatibility(),
    })
}

pub(super) fn encode_presentation(
    revision: u64,
    target: impl Into<Vec<u8>>,
    presentation: EguiTextCommandSurfacePresentation,
    style: TextCommandSurfaceStyle,
) -> Result<EguiTextCommandSurfacePresentationToken, serde_json::Error> {
    let target = target.into();
    let identity = format!("{:x}", Sha256::digest(&target));
    let payload = serde_json::to_vec(&RootPresentationWire {
        presentation,
        style,
    })?;
    Ok(EguiTextCommandSurfacePresentationToken::from_encoded(
        revision,
        EguiTextCommandSurfaceHostTargetToken::from_opaque_bytes(identity),
        payload,
    ))
}

pub(super) fn encode_presentation_with_command_families(
    revision: u64,
    target: impl Into<Vec<u8>>,
    presentation: EguiTextCommandSurfacePresentation,
    style: TextCommandSurfaceStyle,
    command_families: EguiTextCommandSurfaceCommandFamilyProjection,
) -> Result<EguiTextCommandSurfacePresentationToken, serde_json::Error> {
    let target = target.into();
    let identity = format!("{:x}", Sha256::digest(&target));
    let payload = serde_json::to_vec(&RootPresentationWireWithCommandFamilies {
        version: COMMAND_FAMILY_WIRE_VERSION,
        presentation,
        style,
        command_families,
    })?;
    Ok(EguiTextCommandSurfacePresentationToken::from_encoded(
        revision,
        EguiTextCommandSurfaceHostTargetToken::from_opaque_bytes(identity),
        payload,
    ))
}
