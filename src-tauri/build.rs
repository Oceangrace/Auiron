mod build_support;
#[allow(dead_code)]
#[path = "src/integrations/aion2/catalog/source/catalog_checksum.rs"]
mod catalog_checksum;
#[allow(dead_code)]
#[path = "src/integrations/aion2/catalog/source/catalog_manifest.rs"]
mod catalog_manifest;
#[allow(dead_code)] // Runtime and build share these definitions; each uses its own getters.
#[path = "src/config/definition/mod.rs"]
mod definition;
#[allow(dead_code)]
#[path = "src/integrations/aion2/catalog/source/zone_catalog.rs"]
mod zone_catalog;

pub(crate) use self::catalog_checksum::CatalogChecksum;

fn main() {
    build_support::ResourceCompiler::compile();
}
