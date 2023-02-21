use log::debug;

use crate::patch::PatchManager;

use super::CommandExecutor;

pub struct RestoreCommandExecutor;

impl CommandExecutor for RestoreCommandExecutor {
    fn invoke(&self, _args: &[String]) -> std::io::Result<i32> {
        let mut patch_manager = PatchManager::new()?;
        debug!("Handle Command \"restore\"");

        patch_manager.restore_all_patch_status()?;

        debug!("Command \"restore\" done");
        Ok(0)
    }
}
