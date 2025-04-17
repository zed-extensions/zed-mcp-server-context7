use std::env;
use zed_extension_api::{self as zed, Command, ContextServerId, Project, Result};

const PACKAGE_NAME: &str = "@upstash/context7-mcp";
const PACKAGE_VERSION: &str = "latest";
const SERVER_PATH: &str = "node_modules/@upstash/context7-mcp/dist/index.js";

struct Context7ModelContextExtension;

impl zed::Extension for Context7ModelContextExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Command> {
        let version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if version.as_deref() != Some(PACKAGE_VERSION) {
            zed::npm_install_package(PACKAGE_NAME, PACKAGE_VERSION)?;
        }
        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![env::current_dir()
                .unwrap()
                .join(SERVER_PATH)
                .to_string_lossy()
                .to_string()],
            env: vec![],
        })
    }
}

zed::register_extension!(Context7ModelContextExtension);
