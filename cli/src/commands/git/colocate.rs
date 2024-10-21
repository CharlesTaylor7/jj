// Copyright 2020-2023 The Jujutsu Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fs;
use std::io;
use std::io::Write;
use std::num::NonZeroU32;
use std::path::Path;
use std::path::PathBuf;

use jj_lib::git;
use jj_lib::git::GitFetchError;
use jj_lib::git::GitFetchStats;
use jj_lib::repo::Repo;
use jj_lib::str_util::StringPattern;
use jj_lib::workspace::Workspace;

use crate::cli_util::CommandHelper;
use crate::cli_util::WorkspaceCommandHelper;
use crate::command_error::cli_error;
use crate::command_error::user_error;
use crate::command_error::user_error_with_message;
use crate::command_error::CommandError;
use crate::commands::git::map_git_error;
use crate::commands::git::maybe_add_gitignore;
use crate::config::write_config_value_to_file;
use crate::config::ConfigNamePathBuf;
use crate::git_util::get_git_repo;
use crate::git_util::print_git_import_stats;
use crate::git_util::with_remote_git_callbacks;
use crate::ui::Ui;

/// Converts a git backed bare repo to a colocated one
///
/// The Git repo metadata will be stored in `.git/` directory adjacent to the `.jj/` directory.
#[derive(clap::Args, Clone, Debug)]
pub struct GitColocateArgs {}

pub fn cmd_git_colocate(
    ui: &mut Ui,
    command: &CommandHelper,
    args: &GitColocateArgs,
) -> Result<(), CommandError> {
    let workspace = command.workspace_loader()?;

    writeln!(
        ui.status(),
        "repo path:{:#?} \nws root {:#?}",
        workspace.repo_path(),
        workspace.workspace_root()
    )?;

    Ok(())
}
