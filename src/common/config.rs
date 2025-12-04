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

/// Get all possible config paths in order of priority (lowest to highest)
/// Returns: [PROGRAMDATA path, APPDATA path, exe directory path]
fn get_config_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    // Helper function to build config path from a directory
    #[cfg(target_os = "windows")]
    let build_config_path = |dir: String| PathBuf::from(dir).join("win-move").join("config.toml");

    // 1. PROGRAMDATA (system defaults) - lowest priority
    #[cfg(target_os = "windows")]
    if let Ok(programdata) = std::env::var("PROGRAMDATA") {
        paths.push(build_config_path(programdata));
    }

    // 2. APPDATA (user overrides) - medium priority
    #[cfg(target_os = "windows")]
    if let Ok(appdata) = std::env::var("APPDATA") {
        paths.push(build_config_path(appdata));
    }

    // 3. Exe directory (portable mode or local overrides) - highest priority
    // Note: current_exe().unwrap_or_default() returns empty PathBuf on failure,
    // which causes parent() to return None, falling back to current directory "."
    let exe_path = std::env::current_exe().unwrap_or_default();
    let exe_dir = exe_path.parent().unwrap_or(std::path::Path::new("."));
    paths.push(exe_dir.join("config.toml"));

    paths
}

/// Load a config file from a specific path
fn load_config_from_path(path: &PathBuf) -> Option<Config> {
    if let Ok(contents) = fs::read_to_string(path) {
        match toml::from_str(&contents) {
            Ok(config) => {
                log::info!("Loaded configuration from {:?}", path);
                return Some(config);
            }
            Err(e) => {
                log::warn!(
                    "Failed to parse config file at {:?}: {}. Skipping this file.",
                    path,
                    e
                );
            }
        }
    }
    None
}

/// Merge two configs, with the second config overriding the first
/// Hotkeys in the override config will replace hotkeys with the same key+modifier combination
/// from the base config, while preserving other base hotkeys
fn merge_configs(base: Config, override_config: Config) -> Config {
    let mut hotkeys = base.hotkeys;

    // For each override hotkey, either replace an existing one with the same key+modifier
    // or add it as a new hotkey
    for override_hotkey in override_config.hotkeys {
        // Find if there's an existing hotkey with the same key+modifier combination
        if let Some(pos) = hotkeys
            .iter()
            .position(|h| h.key == override_hotkey.key && h.modifier == override_hotkey.modifier)
        {
            // Replace the existing hotkey
            hotkeys[pos] = override_hotkey;
        } else {
            // Add as a new hotkey
            hotkeys.push(override_hotkey);
        }
    }

    Config { hotkeys }
}

pub fn load_config() -> Config {
    let paths = get_config_paths();
    let mut config = Config::default();
    let mut loaded_any = false;

    // Load configs in order of priority, merging as we go
    for path in paths {
        if let Some(loaded_config) = load_config_from_path(&path) {
            config = merge_configs(config, loaded_config);
            loaded_any = true;
        }
    }

    if !loaded_any {
        log::info!("No config files found. Using default configuration.");
    }

    config
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

    #[test]
    fn test_merge_configs() {
        let base = Config {
            hotkeys: vec![
                HotkeyMapping {
                    action: HotKeyAction::MoveWindowToLeftBottom,
                    key: HotKeyButton::VkNumpad1,
                    modifier: HotKeyModifier::ModControl,
                },
                HotkeyMapping {
                    action: HotKeyAction::MoveWindowToTop,
                    key: HotKeyButton::VkNumpad8,
                    modifier: HotKeyModifier::ModControl,
                },
            ],
        };

        let override_config = Config {
            hotkeys: vec![HotkeyMapping {
                action: HotKeyAction::MaximizeWindow,
                key: HotKeyButton::VkNumpad5,
                modifier: HotKeyModifier::ModControl,
            }],
        };

        let merged = merge_configs(base, override_config);
        // Should have 3 hotkeys: 2 from base + 1 new from override
        assert_eq!(merged.hotkeys.len(), 3);
        // Check that all are present
        assert!(merged
            .hotkeys
            .iter()
            .any(|h| h.action == HotKeyAction::MoveWindowToLeftBottom));
        assert!(merged
            .hotkeys
            .iter()
            .any(|h| h.action == HotKeyAction::MoveWindowToTop));
        assert!(merged
            .hotkeys
            .iter()
            .any(|h| h.action == HotKeyAction::MaximizeWindow));
    }

    #[test]
    fn test_merge_configs_replace_existing() {
        let base = Config {
            hotkeys: vec![
                HotkeyMapping {
                    action: HotKeyAction::MoveWindowToLeftBottom,
                    key: HotKeyButton::VkNumpad1,
                    modifier: HotKeyModifier::ModControl,
                },
                HotkeyMapping {
                    action: HotKeyAction::MoveWindowToTop,
                    key: HotKeyButton::VkNumpad8,
                    modifier: HotKeyModifier::ModControl,
                },
            ],
        };

        // Override the VkNumpad1 + ModControl binding with a different action
        let override_config = Config {
            hotkeys: vec![HotkeyMapping {
                action: HotKeyAction::MaximizeWindow,
                key: HotKeyButton::VkNumpad1,
                modifier: HotKeyModifier::ModControl,
            }],
        };

        let merged = merge_configs(base, override_config);
        // Should still have 2 hotkeys (one was replaced)
        assert_eq!(merged.hotkeys.len(), 2);
        // The VkNumpad1 binding should now be MaximizeWindow, not MoveWindowToLeftBottom
        let numpad1_hotkey = merged
            .hotkeys
            .iter()
            .find(|h| h.key == HotKeyButton::VkNumpad1)
            .unwrap();
        assert_eq!(numpad1_hotkey.action, HotKeyAction::MaximizeWindow);
        // VkNumpad8 should still be there
        assert!(merged
            .hotkeys
            .iter()
            .any(|h| h.action == HotKeyAction::MoveWindowToTop));
    }

    #[test]
    fn test_merge_configs_empty_override() {
        let base = Config {
            hotkeys: vec![HotkeyMapping {
                action: HotKeyAction::MoveWindowToLeftBottom,
                key: HotKeyButton::VkNumpad1,
                modifier: HotKeyModifier::ModControl,
            }],
        };

        let override_config = Config { hotkeys: vec![] };

        let merged = merge_configs(base, override_config);
        // Empty override should keep base unchanged
        assert_eq!(merged.hotkeys.len(), 1);
        assert_eq!(
            merged.hotkeys[0].action,
            HotKeyAction::MoveWindowToLeftBottom
        );
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_get_config_paths_windows() {
        let paths = get_config_paths();
        // Should have at least exe directory path, possibly PROGRAMDATA and APPDATA
        assert!(!paths.is_empty());
        // Last path should be exe directory
        assert!(paths.last().unwrap().ends_with("config.toml"));
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn test_get_config_paths_non_windows() {
        let paths = get_config_paths();
        // Should have only exe directory path on non-Windows
        assert_eq!(paths.len(), 1);
        assert!(paths[0].ends_with("config.toml"));
    }
}
