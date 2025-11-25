//! The views module contains the components for all Layouts and Routes for our app. Each layout and route in our [`Route`]
//! enum will render one of these components.
//!
//! The [`Navbar`] component will be rendered on all pages of our app since every page is under the layout. The layout defines
//! a common wrapper around all child routes.

mod home;
pub use home::Home;

mod blog;
pub use blog::Blog;

mod download;
pub use download::Download;

mod lan;
pub use lan::Lan;

mod more;
pub use more::More;

mod settings;
pub use settings::Settings;

mod navbar;
pub use navbar::Navbar;

mod instance_settings;
pub use instance_settings::{
    InstanceSettingsLayout, InstanceBasic, InstanceMods, InstanceWorlds, 
    InstanceResources, InstanceShaders, InstanceScreenshots, InstanceExport, InstanceModify
};
