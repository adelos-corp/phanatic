//! Backend-neutral contracts for PHANATIC runtime integrations.
//!
//! This crate intentionally does not launch processes. Concrete runtime adapters
//! implement these types and are supervised by phanatic-core.

use serde::{Deserialize, Serialize};
use std::{fmt, path::PathBuf};

/// Stable identifier for a runtime-managed virtual device.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DeviceId(String);

impl DeviceId {
    /// Construct an ID after validating its allowed characters.
    pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        let value = value.into();
        if value.is_empty()
            || value.len() > 64
            || !value
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'))
        {
            return Err(ValidationError(
                "device ID must be 1-64 characters using ASCII letters, digits, '.', '_' or '-'"
                    .to_owned(),
            ));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DeviceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Android runtime lifecycle as observed by PHANATIC.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Failed,
    Unknown,
}

/// Basic virtual-device configuration. Runtime-specific options belong in a
/// future explicitly-versioned extension rather than opaque UI arguments.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DeviceProfile {
    pub id: DeviceId,
    pub display_name: String,
    pub android_api_level: Option<u32>,
    pub width_px: u32,
    pub height_px: u32,
    pub density_dpi: u32,
    pub ram_mb: u32,
}

impl DeviceProfile {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.display_name.trim().is_empty() || self.display_name.len() > 100 {
            return Err(ValidationError(
                "display name must be between 1 and 100 characters".to_owned(),
            ));
        }
        if !(320..=7680).contains(&self.width_px)
            || !(320..=7680).contains(&self.height_px)
        {
            return Err(ValidationError(
                "display dimensions must each be between 320 and 7680 pixels".to_owned(),
            ));
        }
        if !(72..=1000).contains(&self.density_dpi) {
            return Err(ValidationError(
                "density must be between 72 and 1000 dpi".to_owned(),
            ));
        }
        if !(512..=131_072).contains(&self.ram_mb) {
            return Err(ValidationError(
                "memory must be between 512 and 131072 MiB".to_owned(),
            ));
        }
        Ok(())
    }
}

/// Capabilities exposed by a runtime implementation.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuntimeCapabilities {
    pub can_list_devices: bool,
    pub can_start_stop: bool,
    pub can_install_apk: bool,
    pub can_capture_logs: bool,
    pub can_collect_metrics: bool,
    pub can_snapshot: bool,
    pub supports_accelerated_graphics: bool,
}

/// Location and identity of a locally installed runtime executable.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuntimeInstallation {
    pub executable: PathBuf,
    pub version: Option<String>,
}

/// Sanitized runtime error categories suitable for presentation to the UI.
/// Concrete adapters may retain detailed diagnostic context in local logs.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeErrorKind {
    NotInstalled,
    InvalidConfiguration,
    DeviceNotFound,
    AlreadyRunning,
    NotRunning,
    LaunchFailed,
    CommunicationFailed,
    UnsupportedOperation,
    Internal,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuntimeError {
    pub kind: RuntimeErrorKind,
    pub message: String,
}

impl RuntimeError {
    pub fn new(kind: RuntimeErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl std::error::Error for RuntimeError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationError(pub String);

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ValidationError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_profile() -> DeviceProfile {
        DeviceProfile {
            id: DeviceId::new("phanatic-dev-1").unwrap(),
            display_name: "Development Device".to_owned(),
            android_api_level: Some(35),
            width_px: 1080,
            height_px: 1920,
            density_dpi: 420,
            ram_mb: 4096,
        }
    }

    #[test]
    fn accepts_valid_device_id() {
        assert_eq!(DeviceId::new("device_1").unwrap().as_str(), "device_1");
    }

    #[test]
    fn rejects_unsafe_device_id_characters() {
        assert!(DeviceId::new("../device").is_err());
        assert!(DeviceId::new("device name").is_err());
    }

    #[test]
    fn validates_device_profile() {
        assert!(sample_profile().validate().is_ok());
    }

    #[test]
    fn rejects_invalid_display_dimensions() {
        let mut profile = sample_profile();
        profile.width_px = 10;
        assert!(profile.validate().is_err());
    }

    #[test]
    fn capabilities_default_to_unavailable() {
        let capabilities = RuntimeCapabilities::default();
        assert!(!capabilities.can_start_stop);
        assert!(!capabilities.supports_accelerated_graphics);
    }
}
