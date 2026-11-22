use crate::lang;
use sp_pages::{Texts, runtime::command};
use std::{path::PathBuf, sync::Arc};
use strawberry_plugin_api::{
    Context, Plugin, PluginMetadata, Result,
    permission::{Permission, PermissionDefault},
    permissions, register_plugin,
};

const USE: &str = "info-lite:use";

struct InfoLite;

impl Plugin for InfoLite {
    fn new() -> Self {
        Self
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "info-lite".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: vec!["Strawberry Interactive LLC".into()],
            description: lang::text("en", "description", &[]),
            dependencies: vec![],
            permissions: vec![
                permissions::FS_READ_DATA.into(),
                permissions::FS_WRITE_DATA.into(),
            ],
        }
    }

    fn on_load(&self, context: Context) -> Result<()> {
        let dir = PathBuf::from(context.get_data_folder());
        std::fs::create_dir_all(&dir).map_err(|_| lang::text("en", "data_error", &[]))?;
        context.register_permission(&Permission {
            node: USE.into(),
            description: lang::text("en", "permission_use", &[]),
            default: PermissionDefault::Allow,
            children: vec![],
        })?;
        // The owner's files are read here, once; a command reads memory only.
        let texts = Texts::load(&dir, "info").map_err(|key| lang::text("en", key, &[]))?;
        context.register_command(
            command(
                "info",
                &lang::text("en", "command_info", &[]),
                Arc::new(texts),
                lang::text,
            ),
            USE,
        );
        let texts = Texts::load(&dir, "news").map_err(|key| lang::text("en", key, &[]))?;
        context.register_command(
            command(
                "news",
                &lang::text("en", "command_news", &[]),
                Arc::new(texts),
                lang::text,
            ),
            USE,
        );
        Ok(())
    }

    fn on_unload(&self, _context: Context) -> Result<()> {
        Ok(())
    }
}

register_plugin!(InfoLite);
