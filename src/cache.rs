use cmd_lib::run_fun;
use std::error::Error;
use toml::value::Value;

/// Represents a workspace cache which is just a collection of bare repositories.
pub struct Cache {
    pub path: String,
}

impl Cache {
    pub fn cache(&self, workspace_path: String, repo_path: String) -> Result<(), Box<dyn Error>> {
        let source = format!("{}/{}/.git", workspace_path, repo_path);
        let dest = format!("{}/{}", self.path, repo_path);
        run_fun!(mv $source $dest)?;
        set_bare(repo_path, true)?;
        Ok(())
    }

    pub fn uncache(&self, workspace_path: String, repo_path: String) -> Result<(), Box<dyn Error>> {
        let source = format!("{}/{}", self.path, repo_path);
        let dest = format!("{}/{}", workspace_path, repo_path);
        run_fun!(git clone $source $dest)?;
        Ok(())
    }
}

/// Set the "bare" attribute of a .git/config file.
fn set_bare(path: String, bare: bool) -> Result<(), Box<dyn Error>> {
    let mut config: Value = toml::de::from_str(std::fs::read_to_string(&path)?.as_str())?;

    match &mut config {
        Value::Table(table) => match table.get_mut("core") {
            Some(Value::Table(core)) => core.insert("bare".to_string(), Value::Boolean(bare)),
            _ => todo!(),
        },
        _ => todo!(),
    };

    std::fs::write(&path, toml::to_string_pretty(&config)?)?;
    Ok(())
}
