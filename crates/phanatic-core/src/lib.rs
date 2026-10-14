//! PHANATIC orchestration core.
//!
//! The first milestone establishes typed device state and configuration. No
//! emulator process is launched until a runtime adapter has been implemented
//! and its executable/configuration have been verified.

use phanatic_runtime_api::{DeviceId, DeviceProfile, DeviceState, ValidationError};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// In-memory registry used by the first prototype. Persistence and runtime
/// process supervision will be added behind this API in subsequent changes.
#[derive(Debug, Default)]
pub struct DeviceRegistry {
    devices: BTreeMap<DeviceId, RegisteredDevice>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RegisteredDevice {
    pub profile: DeviceProfile,
    pub state: DeviceState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RegistryError {
    DuplicateDevice(DeviceId),
    DeviceNotFound(DeviceId),
    InvalidProfile(String),
}

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateDevice(id) => write!(f, "device '{id}' already exists"),
            Self::DeviceNotFound(id) => write!(f, "device '{id}' was not found"),
            Self::InvalidProfile(message) => write!(f, "invalid device profile: {message}"),
        }
    }
}

impl std::error::Error for RegistryError {}

impl From<ValidationError> for RegistryError {
    fn from(value: ValidationError) -> Self {
        Self::InvalidProfile(value.to_string())
    }
}

impl DeviceRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, profile: DeviceProfile) -> Result<(), RegistryError> {
        profile.validate()?;
        if self.devices.contains_key(&profile.id) {
            return Err(RegistryError::DuplicateDevice(profile.id));
        }

        self.devices.insert(
            profile.id.clone(),
            RegisteredDevice {
                profile,
                state: DeviceState::Stopped,
            },
        );
        Ok(())
    }

    pub fn list(&self) -> impl Iterator<Item = &RegisteredDevice> {
        self.devices.values()
    }

    pub fn get(&self, id: &DeviceId) -> Result<&RegisteredDevice, RegistryError> {
        self.devices
            .get(id)
            .ok_or_else(|| RegistryError::DeviceNotFound(id.clone()))
    }

    /// Update state only for a registered device. Runtime transition validation
    /// will be enforced when process supervision is introduced.
    pub fn set_state(
        &mut self,
        id: &DeviceId,
        state: DeviceState,
    ) -> Result<(), RegistryError> {
        let device = self
            .devices
            .get_mut(id)
            .ok_or_else(|| RegistryError::DeviceNotFound(id.clone()))?;
        device.state = state;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use phanatic_runtime_api::DeviceId;

    fn profile(id: &str) -> DeviceProfile {
        DeviceProfile {
            id: DeviceId::new(id).unwrap(),
            display_name: format!("Device {id}"),
            android_api_level: Some(35),
            width_px: 1080,
            height_px: 1920,
            density_dpi: 420,
            ram_mb: 4096,
        }
    }

    #[test]
    fn registers_and_lists_devices() {
        let mut registry = DeviceRegistry::new();
        registry.register(profile("device-a")).unwrap();
        assert_eq!(registry.list().count(), 1);
    }

    #[test]
    fn rejects_duplicate_device_ids() {
        let mut registry = DeviceRegistry::new();
        registry.register(profile("device-a")).unwrap();
        assert!(matches!(
            registry.register(profile("device-a")),
            Err(RegistryError::DuplicateDevice(_))
        ));
    }

    #[test]
    fn device_state_can_be_updated() {
        let mut registry = DeviceRegistry::new();
        let id = DeviceId::new("device-a").unwrap();
        registry.register(profile("device-a")).unwrap();
        registry.set_state(&id, DeviceState::Running).unwrap();
        assert_eq!(registry.get(&id).unwrap().state, DeviceState::Running);
    }

    #[test]
    fn rejects_unknown_device() {
        let registry = DeviceRegistry::new();
        let id = DeviceId::new("missing").unwrap();
        assert!(matches!(
            registry.get(&id),
            Err(RegistryError::DeviceNotFound(_))
        ));
    }
}
