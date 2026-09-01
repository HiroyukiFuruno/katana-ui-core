use super::super::command_chrome_types::EguiCommandChromeDrawLayer;
use super::{CommandChromePaintOperation, CommandChromePaintOperationKind, CommandChromePaintPlan};
use crate::render_model::UiRect;
use sha2::{Digest, Sha256};

const ASCII_BACKSPACE: u8 = 0x08;
const ASCII_FORM_FEED: u8 = 0x0c;
const ASCII_NUL: u8 = 0x00;
const ASCII_UNIT_SEPARATOR: u8 = 0x1f;
const HEX_DIGIT_COUNT: usize = 16;
const HEX_HIGH_NIBBLE_SHIFT: u8 = 4;
const HEX_LOW_NIBBLE_MASK: u8 = 0x0f;
const HEX: &[u8; HEX_DIGIT_COUNT] = b"0123456789abcdef";

pub(super) fn frame_record_hash(value: &impl std::fmt::Debug) -> String {
    /* WHY: hashはopaqueな同値性証跡でありwire payloadではない。closed typed DTOのDebug表現を
    入力にすることで、到達不能なserialization failureを描画APIへ漏らさず決定性を保つ。 */
    hex::encode(Sha256::digest(format!("{value:?}").as_bytes()))
}

pub(super) fn paint_plan_hash(plan: &CommandChromePaintPlan) -> String {
    hex::encode(Sha256::digest(paint_plan_json(plan)))
}

pub(super) fn paint_plan_json(plan: &CommandChromePaintPlan) -> Vec<u8> {
    let mut output = b"{\"surface_bounds\":".to_vec();
    write_rect(&mut output, plan.surface_bounds);
    output.extend_from_slice(b",\"operations\":[");
    for (index, operation) in plan.operations.iter().enumerate() {
        if index > 0 {
            output.push(b',');
        }
        write_operation(&mut output, operation);
    }
    output.extend_from_slice(b"]}");
    output
}

fn write_operation(output: &mut Vec<u8>, operation: &CommandChromePaintOperation) {
    output.extend_from_slice(b"{\"layer\":");
    write_layer(output, operation.layer);
    output.extend_from_slice(b",\"clip_bounds\":");
    write_rect(output, operation.clip_bounds);
    output.extend_from_slice(b",\"kind\":");
    write_operation_kind(output, &operation.kind);
    output.push(b'}');
}

fn write_layer(output: &mut Vec<u8>, layer: EguiCommandChromeDrawLayer) {
    let name = match layer {
        EguiCommandChromeDrawLayer::PanelFill => "PanelFill",
        EguiCommandChromeDrawLayer::PanelBorder => "PanelBorder",
        EguiCommandChromeDrawLayer::ActionFill => "ActionFill",
        EguiCommandChromeDrawLayer::IconTexture => "IconTexture",
        EguiCommandChromeDrawLayer::TextTexture => "TextTexture",
        EguiCommandChromeDrawLayer::FocusRing => "FocusRing",
        EguiCommandChromeDrawLayer::TooltipFill => "TooltipFill",
        EguiCommandChromeDrawLayer::TooltipTexture => "TooltipTexture",
    };
    write_json_string(output, name);
}

fn write_operation_kind(output: &mut Vec<u8>, kind: &CommandChromePaintOperationKind) {
    match kind {
        CommandChromePaintOperationKind::Fill { bounds, color_rgba } => {
            output.extend_from_slice(b"{\"Fill\":{\"bounds\":");
            write_rect(output, *bounds);
            output.extend_from_slice(b",\"color_rgba\":");
            write_u8_values(output, color_rgba);
            output.extend_from_slice(b"}}");
        }
        CommandChromePaintOperationKind::RoundedFill {
            bounds,
            color_rgba,
            radius_px,
        } => {
            output.extend_from_slice(b"{\"RoundedFill\":{\"bounds\":");
            write_rect(output, *bounds);
            output.extend_from_slice(b",\"color_rgba\":");
            write_u8_values(output, color_rgba);
            output.extend_from_slice(b",\"radius_px\":");
            output.extend_from_slice(radius_px.to_string().as_bytes());
            output.extend_from_slice(b"}}");
        }
        CommandChromePaintOperationKind::Texture { bounds, texture } => {
            output.extend_from_slice(b"{\"Texture\":{\"bounds\":");
            write_rect(output, *bounds);
            output.extend_from_slice(b",\"texture\":{\"identity\":");
            write_json_string(output, &texture.identity);
            output.extend_from_slice(b",\"width\":");
            output.extend_from_slice(texture.width.to_string().as_bytes());
            output.extend_from_slice(b",\"height\":");
            output.extend_from_slice(texture.height.to_string().as_bytes());
            output.extend_from_slice(b",\"rgba_pixels\":");
            write_u8_values(output, &texture.rgba_pixels);
            output.extend_from_slice(b"}}}");
        }
    }
}

fn write_rect(output: &mut Vec<u8>, rect: UiRect) {
    output.extend_from_slice(b"{\"x\":");
    output.extend_from_slice(rect.x.to_string().as_bytes());
    output.extend_from_slice(b",\"y\":");
    output.extend_from_slice(rect.y.to_string().as_bytes());
    output.extend_from_slice(b",\"width\":");
    output.extend_from_slice(rect.width.to_string().as_bytes());
    output.extend_from_slice(b",\"height\":");
    output.extend_from_slice(rect.height.to_string().as_bytes());
    output.push(b'}');
}

fn write_u8_values(output: &mut Vec<u8>, values: &[u8]) {
    output.push(b'[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(b',');
        }
        output.extend_from_slice(value.to_string().as_bytes());
    }
    output.push(b']');
}

fn write_json_string(output: &mut Vec<u8>, value: &str) {
    output.push(b'"');
    for byte in value.bytes() {
        match byte {
            b'"' => output.extend_from_slice(b"\\\""),
            b'\\' => output.extend_from_slice(b"\\\\"),
            ASCII_BACKSPACE => output.extend_from_slice(b"\\b"),
            ASCII_FORM_FEED => output.extend_from_slice(b"\\f"),
            b'\n' => output.extend_from_slice(b"\\n"),
            b'\r' => output.extend_from_slice(b"\\r"),
            b'\t' => output.extend_from_slice(b"\\t"),
            ASCII_NUL..=ASCII_UNIT_SEPARATOR => {
                output.extend_from_slice(b"\\u00");
                output.push(HEX[usize::from(byte >> HEX_HIGH_NIBBLE_SHIFT)]);
                output.push(HEX[usize::from(byte & HEX_LOW_NIBBLE_MASK)]);
            }
            _ => output.push(byte),
        }
    }
    output.push(b'"');
}
