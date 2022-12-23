# Cilki's Git WorkSpace Manager

`wsm` is a command line utility for quickly cloning (and decloning) Git repositories.

### Workspaces

Simply defined, a _working set_ is the set of all repositories you're able to work
on at any given time. Even more simply, your _workspace_ is just the local directory
where you keep your Git repositories.

The principle behind `wsm` is that your Git workspace should only consist of your
_working set_. This reduces unnecessary noise as repositories accumulate in your
workspace over time and improves indexing performance of your development tools.
Adhering to this principle involves frequently cloning and deleting repositories
from your workspace, which is exactly what `wsm` is designed to automate.

When repositories are dropped from your workspace, they are cached locally so
restoring them later can be done in an instant.

### Basic Usage

```sh
# Add or refresh a repository in the default workspace
wsm github.com/cilki/wsm

# The repository's local path always reflects the remote path for identifiability
cd ~/workspace/github.com/cilki/wsm

# Remove everything I'm not currently working on from the workspace (any repo
# that has no unstaged or unpushed changes).
wsm drop

# Or, you can also select repositories to drop
wsm drop github.com/cilki/*

# If you need to work on a repository again, it's restored from cache which is
# super fast!
wsm github.com/cilki/wsm
```

### Basic Configuration

`wsm` attempts to read a configuration file from `~/.wsm/config.toml`. For example:

```toml
# Define any number of workspaces
[[workspace]]
name = "personal"
path = "/home/user/workspace"

[[workspace]]
name = "work"
path = "/home/worker/workspace"
```

