use serde::{Deserialize, Serialize};

use crate::types::{CustomEmojiId, Rgb};

/// This object contains information about the color scheme for a user's name,
/// message replies and link previews based on a unique gift.
///
/// [The official docs](https://core.telegram.org/bots/api#uniquegiftcolors).
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct UniqueGiftColors {
    /// Custom emoji identifier of the unique gift's model.
    pub model_custom_emoji_id: CustomEmojiId,

    /// Custom emoji identifier of the unique gift's symbol.
    pub symbol_custom_emoji_id: CustomEmojiId,

    /// Main color used in light themes; RGB format.
    pub light_theme_main_color: Rgb,

    /// List of 1-3 additional colors used in light themes; RGB format.
    pub light_theme_other_colors: Vec<Rgb>,

    /// Main color used in dark themes; RGB format.
    pub dark_theme_main_color: Rgb,

    /// List of 1-3 additional colors used in dark themes; RGB format.
    pub dark_theme_other_colors: Vec<Rgb>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let json = r#"{
            "model_custom_emoji_id": "5404607081052758526",
            "symbol_custom_emoji_id": "5404607081052758527",
            "light_theme_main_color": 16776960,
            "light_theme_other_colors": [16776960, 65280],
            "dark_theme_main_color": 255,
            "dark_theme_other_colors": [0]
        }"#;

        let colors: UniqueGiftColors = serde_json::from_str(json).unwrap();

        assert_eq!(
            colors,
            UniqueGiftColors {
                model_custom_emoji_id: CustomEmojiId("5404607081052758526".to_owned()),
                symbol_custom_emoji_id: CustomEmojiId("5404607081052758527".to_owned()),
                light_theme_main_color: Rgb { r: 0xFF, g: 0xFF, b: 0x00 },
                light_theme_other_colors: vec![
                    Rgb { r: 0xFF, g: 0xFF, b: 0x00 },
                    Rgb { r: 0x00, g: 0xFF, b: 0x00 },
                ],
                dark_theme_main_color: Rgb { r: 0x00, g: 0x00, b: 0xFF },
                dark_theme_other_colors: vec![Rgb { r: 0x00, g: 0x00, b: 0x00 }],
            }
        );
    }
}
