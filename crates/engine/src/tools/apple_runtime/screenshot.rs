//! Bounded PNGs of an already-running window, shared with the Windows runtime reader.

use crate::tools::web_common::{Image, encode_png_within, png_artifact, resize};
use craft_anything_evidence::{ErrorCode, ToolError};
use craft_anything_platform::window::WindowShot;
use serde_json::Value;

pub(super) fn artifact(shot: WindowShot) -> Result<Value, ToolError> {
    let pixels = u64::from(shot.width) * u64::from(shot.height);
    if pixels == 0 || pixels > 64 * 1024 * 1024 || pixels.checked_mul(4) != Some(shot.rgba.len() as u64) {
        return Err(ToolError::new(ErrorCode::OverLimit, "The window screenshot has invalid or excessive dimensions."));
    }
    let mut image = Image { width: shot.width, height: shot.height, rgba: shot.rgba };
    let longest = image.width.max(image.height);
    if longest > 2048 {
        let width = (u64::from(image.width) * 2048 / u64::from(longest)).max(1) as u32;
        let height = (u64::from(image.height) * 2048 / u64::from(longest)).max(1) as u32;
        image = resize(&image, width, height);
    }
    let png = encode_png_within(&image, 2 * 1024 * 1024)?
        .ok_or_else(|| ToolError::new(ErrorCode::OverLimit, "The window screenshot exceeded the 2 MiB PNG limit."))?;
    let mut artifact = png_artifact(&png)?;
    if let Some(object) = artifact.as_object_mut() {
        object.insert("title".into(), Value::String(shot.title));
        object.insert("app".into(), Value::String(shot.app));
    }
    Ok(artifact)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn screenshots_are_pngs_with_bounded_dimensions_and_matching_metadata() {
        let shot = WindowShot { title: "Editor".into(), app: "Fixture".into(), width: 4096, height: 8, rgba: vec![255; 4096 * 8 * 4] };
        let output = artifact(shot).unwrap();
        assert_eq!(output["width"], 2048);
        assert_eq!(output["height"], 4);
        assert_eq!(output["title"], "Editor");
        let bytes = crate::tools::web_common::b64_decode(output["base64"].as_str().unwrap()).unwrap();
        assert!(bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
        assert_eq!(bytes.len() as u64, output["bytes"].as_u64().unwrap());
        assert_eq!(craft_anything_evidence::sha256_hex(&bytes), output["sha256"]);
    }

    #[test]
    fn inconsistent_or_excessive_window_buffers_are_refused() {
        for (width, height) in [(0, 10), (10, 10), (u32::MAX, u32::MAX)] {
            let shot = WindowShot { title: String::new(), app: String::new(), width, height, rgba: Vec::new() };
            assert_eq!(artifact(shot).unwrap_err().code, ErrorCode::OverLimit);
        }
    }
}
