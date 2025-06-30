use zed_extension_api as zed;

struct YapExtension;

impl zed::Extension for YapExtension {
    fn new() -> Self {
       Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        Ok(zed::Command {
            command: "yap".into(),
            args: vec!["lsp".into()],
            env: vec![],
        })
    }
}

zed::register_extension!(YapExtension);
