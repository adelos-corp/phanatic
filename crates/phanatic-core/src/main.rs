use phanatic_core::DeviceRegistry;
use phanatic_runtime_api::{DeviceId, DeviceProfile};

fn main() {
    let registry = DeviceRegistry::new();
    println!("PHANATIC core scaffold v{}", env!("CARGO_PKG_VERSION"));
    println!("Registered virtual devices: {}", registry.list().count());
    println!("Runtime integration: not configured");

    // Keep the sample identifier validation visible in the executable until
    // runtime discovery and user-facing configuration are implemented.
    match DeviceId::new("phanatic-default") {
        Ok(id) => println!("Example device ID accepted: {id}"),
        Err(error) => eprintln!("Example device ID rejected: {error}"),
    }

    let example = DeviceProfile {
        id: DeviceId::new("phanatic-default").expect("static ID is valid"),
        display_name: "PHANATIC Default".to_owned(),
        android_api_level: None,
        width_px: 1080,
        height_px: 1920,
        density_dpi: 420,
        ram_mb: 4096,
    };
    if let Err(error) = example.validate() {
        eprintln!("Example profile failed validation: {error}");
    }
}
