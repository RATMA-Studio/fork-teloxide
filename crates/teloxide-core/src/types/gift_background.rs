use serde::{Deserialize, Serialize};

use crate::types::Rgb;

/// This object describes the background of a gift.
///
/// [The official docs](https://core.telegram.org/bots/api#giftbackground).
#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct GiftBackground {
    /// Center color of the background in RGB format.
    pub center_color: Rgb,

    /// Edge color of the background in RGB format.
    pub edge_color: Rgb,

    /// Text color of the background in RGB format.
    pub text_color: Rgb,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let json = r#"{
            "center_color": 16776960,
            "edge_color": 65280,
            "text_color": 255
        }"#;

        let bg: GiftBackground = serde_json::from_str(json).unwrap();

        assert_eq!(
            bg,
            GiftBackground {
                center_color: Rgb { r: 0xFF, g: 0xFF, b: 0x00 },
                edge_color: Rgb { r: 0x00, g: 0xFF, b: 0x00 },
                text_color: Rgb { r: 0x00, g: 0x00, b: 0xFF },
            }
        );
    }
}
