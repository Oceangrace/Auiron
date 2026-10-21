use crate::catalog_manifest::CatalogManifest;
use crate::definition::ApplicationDefinition;
use crate::zone_catalog::ZoneCatalog;
use serde_json::{Value, json};
use std::path::Path;

pub(crate) struct ResourceCompiler;

impl ResourceCompiler {
    fn read(path: &Path) -> Value {
        println!("cargo:rerun-if-changed={}", path.display());

        serde_json::from_slice(&std::fs::read(path).expect("Read configured JSON"))
            .expect("Parse configured JSON")
    }

    pub(crate) fn compile() {
        if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
            panic!("Aurion supports Windows only");
        }

        let project = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("Project directory");

        let configuration_path = project.join("data/application.json");

        println!("cargo:rerun-if-changed={}", configuration_path.display());
        println!(
            "cargo:rustc-env=AURION_CONFIGURATION={}",
            configuration_path.display()
        );
        println!("cargo:rustc-env=AURION_PROJECT_ROOT={}", project.display());

        let config = ApplicationDefinition::parse(
            &std::fs::read(&configuration_path).expect("Read application configuration"),
        )
        .expect("Validate application configuration");

        let data_root = configuration_path
            .parent()
            .expect("Configuration directory");

        let protocol = data_root.join(config.protocol().table());

        println!("cargo:rerun-if-changed={}", protocol.display());
        println!(
            "cargo:rustc-env=AURION_PROTOCOL_TABLE={}",
            protocol.display()
        );

        let catalog = config.catalog();

        for (key, name) in [
            ("MANIFEST", catalog.manifest()),
            ("PLAYER", catalog.player()),
            ("ENEMY", catalog.enemy()),
            ("ZONES", catalog.zones()),
            ("NPC", catalog.npcs()),
            ("RUNTIME", catalog.runtime()),
            ("SKILL_NAMES", catalog.skill_names()),
        ] {
            let file = data_root.join(catalog.directory()).join(name);

            println!("cargo:rerun-if-changed={}", file.display());
            println!("cargo:rustc-env=AURION_CATALOG_{key}={}", file.display());
        }

        let manifest = CatalogManifest::parse(
            &std::fs::read(data_root.join(catalog.directory()).join(catalog.manifest()))
                .expect("Read catalog manifest"),
        )
        .expect("Validate catalog manifest");

        for name in [
            catalog.player(),
            catalog.enemy(),
            catalog.zones(),
            catalog.npcs(),
            catalog.runtime(),
            catalog.skill_names(),
        ] {
            let file = data_root.join(catalog.directory()).join(name);

            manifest
                .verify(name, &std::fs::read(file).expect("Read catalog file"))
                .expect("Verify catalog checksum");
        }

        let icon = project.join(config.resources().tray_icon());

        println!("cargo:rerun-if-changed={}", icon.display());
        println!("cargo:rustc-env=AURION_TRAY_ICON={}", icon.display());

        let zone_path = data_root.join(catalog.directory()).join(catalog.zones());

        let zone_data = ZoneCatalog::parse(
            &std::fs::read(&zone_path).expect("Read configured zones"),
            catalog.client_build(),
            manifest.version(),
        )
        .expect("Parse configured zones");

        let packs: Vec<Value> = config
            .localization()
            .bundled()
            .iter()
            .map(|locale| {
                let ui = Self::read(&data_root.join(locale.ui()));

                let texts = locale.catalog().map_or(Value::Null, |path| {
                    let name = path
                        .strip_prefix(catalog.directory())
                        .expect("Bundled catalog translation must be inside the catalog directory");

                    let file = data_root.join(path);

                    manifest
                        .verify(
                            name,
                            &std::fs::read(&file).expect("Read catalog translation"),
                        )
                        .expect("Verify catalog translation checksum");

                    Self::read(&file)
                });

                let zones = texts.get("zones").cloned().unwrap_or(Value::Null);

                if locale.catalog().is_some() {
                    let names = zones
                        .as_object()
                        .expect("Bundled zone translations must be an object");

                    assert!(
                        zone_data
                            .values()
                            .iter()
                            .all(|name| names.get(name).is_some_and(Value::is_string)),
                        "Missing bundled zone translation"
                    );
                }

                json!({
                    "id": locale.id(),
                    "label": locale.label(),
                    "messages": ui,
                    "catalog": texts,
                    "zones": zones,
                })
            })
            .collect();

        let output = std::path::PathBuf::from(std::env::var_os("OUT_DIR").expect("Build output"));

        std::fs::write(
            output.join("locales.json"),
            serde_json::to_vec(&packs).expect("Serialize bundled locales"),
        )
        .expect("Write bundled locales");
        println!(
            "cargo:rustc-env=AURION_BUNDLED_LOCALES={}",
            output.join("locales.json").display()
        );
        tauri_build::build();
    }
}
