use crate::common::enums::{HotKeyButton, HotKeyModifier};
use crate::common::hotkey_action::HotKeyAction;
use crate::common::structs::HotkeyMapping;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub hotkeys: Vec<HotkeyMapping>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            hotkeys: vec![
                HotkeyMapping {
                    action: HotKeyAction::MoveWindowToLeftBottom,
                    key: HotKeyButton::VkNumpad1,
                    modifier: HotKeyModifier::ModControl,
                },
                HotkeyMapping {
                    action: HotKeyAction::MoveWindowToBottom,
                    key: HotKeyButton::VkNumpad2,
                    modifier: HotKeyModifier::ModControl,
                },
                HotkeyMapping {
                    action: HotKeyAction::MoveWindowToRightBottom,
                    key: HotKeyButton::VkNumpad3,
                    modifier: HotKeyModifier::ModControl,
                },
                HotkeyMapping {
                    action: HotKeyAction::MoveWindowToLeftMiddle,
                    key: HotKeyButton::VkNumpad4,
                    modifier: HotKeyModifier::ModControl,
                },
                HotkeyMapping {
                    action: HotKeyAction::MoveWindowToRightMiddle,
                    key: HotKeyButton::VkNumpad6,
                    modifier: HotKeyModifier::ModControl,
                },
                HotkeyMapping {
                    action: HotKeyAction::MoveWindowToLeftTop,
                    key: HotKeyButton::VkNumpad7,
                    modifier: HotKeyModifier::ModControl,
                },
                HotkeyMapping {
                    action: HotKeyAction::MoveWindowToTop,
                    key: HotKeyButton::VkNumpad8,
                    modifier: HotKeyModifier::ModControl,
                },
                HotkeyMapping {
                    action: HotKeyAction::MoveWindowToRightTop,
                    key: HotKeyButton::VkNumpad9,
                    modifier: HotKeyModifier::ModControl,
                },
                HotkeyMapping {
                    action: HotKeyAction::MoveWindowToLeftScreenContinuous,
                    key: HotKeyButton::VkNumpad0,
                    modifier: HotKeyModifier::ModControl,
                },
                HotkeyMapping {
                    action: HotKeyAction::MinimizeWindow,
                    key: HotKeyButton::VkDecimal,
                    modifier: HotKeyModifier::ModControl,
                },
                HotkeyMapping {
                    action: HotKeyAction::MaximizeWindow,
                    key: HotKeyButton::VkNumpad5,
                    modifier: HotKeyModifier::ModControl,
                },
            ],
        }
    }
}

fn get_config_path() -> PathBuf {
    let exe_path = std::env::current_exe().unwrap_or_default();
    let exe_dir = exe_path.parent().unwrap_or(std::path::Path::new("."));
    exe_dir.join("config.toml")
}

pub fn load_config() -> Config {
    let config_path = get_config_path();

    if let Ok(contents) = fs::read_to_string(&config_path) {
        match toml::from_str(&contents) {
            Ok(config) => {
                log::info!("Loaded configuration from {:?}", config_path);
                return config;
            }
            Err(e) => {
                log::warn!(
                    "Failed to parse config file: {}. Using default configuration.",
                    e
                );
            }
        }
    } else {
        log::info!(
            "No config file found at {:?}. Using default configuration.",
            config_path
        );
    }

    Config::default()
}

pub fn get_config_hotkeys() -> Vec<HotkeyMapping> {
    load_config().hotkeys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_has_hotkeys() {
        let config = Config::default();
        assert!(!config.hotkeys.is_empty());
        assert!(config.hotkeys.len() >= 8);
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let serialized = toml::to_string(&config).unwrap();
        assert!(serialized.contains("hotkeys"));
        assert!(serialized.contains("MoveWindowToLeftBottom"));
    }

    #[test]
    fn test_config_deserialization() {
        let toml_str = r#"
[[hotkeys]]
action = "MoveWindowToLeftBottom"
key = "VkNumpad1"
modifier = "ModControl"

[[hotkeys]]
action = "MoveWindowToTop"
key = "VkNumpad8"
modifier = "ModControl"
"#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.hotkeys.len(), 2);
        assert_eq!(
            config.hotkeys[0].action,
            HotKeyAction::MoveWindowToLeftBottom
        );
        assert_eq!(config.hotkeys[1].action, HotKeyAction::MoveWindowToTop);
    }
}
