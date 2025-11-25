//! Instance settings views module

mod basic;
mod mods;
mod worlds;
mod resources;
mod shaders;
mod screenshots;
mod export;
mod modify;
mod layout;

pub use basic::InstanceBasic;
pub use mods::InstanceMods;
pub use worlds::InstanceWorlds;
pub use resources::InstanceResources;
pub use shaders::InstanceShaders;
pub use screenshots::InstanceScreenshots;
pub use export::InstanceExport;
pub use modify::InstanceModify;
pub use layout::InstanceSettingsLayout;
