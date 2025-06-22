use crate::git::types::{
    CommitHistoryItem, CommitOptions, FileStatus, GitError, PullStrategy, RemoteBranchInfo,
    RepositoryStatus, SyncResult,
};
use git2::{
    FetchOptions, PushOptions, RemoteCallbacks, Repository, Signature, Status, StatusOptions,
};
use std::path::Path;

/// 获取仓库状态
pub fn get_repository_status(repo_path: &str) -> Result<RepositoryStatus, GitError> {
    let repo = Repository::open(repo_path).map_err(GitError::Git)?;

    // 获取状态选项
    let mut status_options = StatusOptions::new();
    status_options.include_untracked(true);
    status_options.include_ignored(false);

    // 获取文件状态
    let statuses = repo
        .statuses(Some(&mut status_options))
        .map_err(GitError::Git)?;

    let mut files = Vec::new();

    for entry in statuses.iter() {
        let path = entry.path().unwrap_or("").to_string();
        let git_status = entry.status();

        // 检查是否有暂存的变更
        let has_staged_changes = is_staged(git_status);

        // 检查是否有未暂存的变更
        let has_unstaged_changes = has_unstaged_changes(git_status);

        // 如果有暂存的变更，添加暂存条目
        if has_staged_changes {
            let status = convert_git_status_staged(git_status);
            let (additions, deletions) = calculate_file_stats(&repo, &path, true)?;

            files.push(FileStatus {
                path: path.clone(),
                status,
                staged: true,
                additions,
                deletions,
            });
        }

        // 如果有未暂存的变更，添加未暂存条目
        if has_unstaged_changes {
            let status = convert_git_status_unstaged(git_status);
            let (additions, deletions) = calculate_file_stats(&repo, &path, false)?;

            files.push(FileStatus {
                path,
                status,
                staged: false,
                additions,
                deletions,
            });
        }
    }

    // 获取当前分支信息
    let current_branch = get_current_branch(&repo)?;

    // 获取远程跟踪信息
    let (ahead, behind) = get_ahead_behind_count(&repo)?;

    let is_clean = files.is_empty();

    Ok(RepositoryStatus {
        current_branch,
        files,
        ahead,
        behind,
        is_clean,
    })
}

/// 暂存文件
pub fn stage_files(repo_path: &str, file_paths: &[String]) -> Result<(), GitError> {
    let repo = Repository::open(repo_path).map_err(GitError::Git)?;
    let mut index = repo.index().map_err(GitError::Git)?;

    for file_path in file_paths {
        // 检查文件是否存在
        let full_path = Path::new(repo_path).join(file_path);
        if full_path.exists() {
            index
                .add_path(Path::new(file_path))
                .map_err(GitError::Git)?;
        } else {
            // 文件被删除，需要从索引中移除
            index
                .remove_path(Path::new(file_path))
                .map_err(GitError::Git)?;
        }
    }

    index.write().map_err(GitError::Git)?;
    Ok(())
}

/// 取消暂存文件
pub fn unstage_files(repo_path: &str, file_paths: &[String]) -> Result<(), GitError> {
    let repo = Repository::open(repo_path).map_err(GitError::Git)?;

    // 获取HEAD提交
    let head = repo.head().map_err(GitError::Git)?;
    let head_commit = head.peel_to_commit().map_err(GitError::Git)?;
    let head_tree = head_commit.tree().map_err(GitError::Git)?;

    let mut index = repo.index().map_err(GitError::Git)?;

    for file_path in file_paths {
        // 从HEAD恢复文件到索引
        if let Ok(entry) = head_tree.get_path(Path::new(file_path)) {
            // 创建一个新的索引条目
            let index_entry = git2::IndexEntry {
                ctime: git2::IndexTime::new(0, 0),
                mtime: git2::IndexTime::new(0, 0),
                dev: 0,
                ino: 0,
                mode: entry.filemode() as u32,
                uid: 0,
                gid: 0,
                file_size: 0,
                id: entry.id(),
                flags: 0,
                flags_extended: 0,
                path: file_path.as_bytes().to_vec(),
            };
            index.add(&index_entry).map_err(GitError::Git)?;
        } else {
            // 文件在HEAD中不存在，从索引中移除
            let _ = index.remove_path(Path::new(file_path));
        }
    }

    index.write().map_err(GitError::Git)?;
    Ok(())
}

/// 创建提交
pub fn create_commit(repo_path: &str, options: &CommitOptions) -> Result<String, GitError> {
    let repo = Repository::open(repo_path).map_err(GitError::Git)?;

    // 获取签名
    let signature = if let (Some(name), Some(email)) = (&options.author_name, &options.author_email)
    {
        Signature::now(name, email).map_err(GitError::Git)?
    } else {
        repo.signature().map_err(GitError::Git)?
    };

    // 获取索引并创建树
    let mut index = repo.index().map_err(GitError::Git)?;
    let tree_id = index.write_tree().map_err(GitError::Git)?;
    let tree = repo.find_tree(tree_id).map_err(GitError::Git)?;

    // 构建提交消息
    let mut message = options.message.clone();
    if let Some(description) = &options.description {
        if !description.is_empty() {
            message.push_str("\n\n");
            message.push_str(description);
        }
    }

    if options.signoff {
        message.push_str(&format!(
            "\n\nSigned-off-by: {} <{}>",
            signature.name().unwrap_or(""),
            signature.email().unwrap_or("")
        ));
    }

    // 获取父提交
    let parents = if options.amend {
        // 修正提交：使用当前HEAD的父提交
        let head = repo.head().map_err(GitError::Git)?;
        let head_commit = head.peel_to_commit().map_err(GitError::Git)?;
        head_commit.parents().collect::<Vec<_>>()
    } else {
        // 正常提交：使用HEAD作为父提交
        match repo.head() {
            Ok(head) => {
                let head_commit = head.peel_to_commit().map_err(GitError::Git)?;
                vec![head_commit]
            }
            Err(_) => {
                // 首次提交，没有父提交
                vec![]
            }
        }
    };

    let parent_refs: Vec<&git2::Commit> = parents.iter().collect();

    // 创建提交
    let commit_id = repo
        .commit(
            Some("HEAD"),
            &signature,
            &signature,
            &message,
            &tree,
            &parent_refs,
        )
        .map_err(GitError::Git)?;

    Ok(commit_id.to_string())
}

/// 获取提交历史
pub fn get_commit_history(
    repo_path: &str,
    limit: usize,
    skip: usize,
) -> Result<Vec<CommitHistoryItem>, GitError> {
    let repo = Repository::open(repo_path).map_err(GitError::Git)?;

    let mut revwalk = repo.revwalk().map_err(GitError::Git)?;
    revwalk.push_head().map_err(GitError::Git)?;
    revwalk
        .set_sorting(git2::Sort::TIME)
        .map_err(GitError::Git)?;

    let mut commits = Vec::new();

    for (index, oid_result) in revwalk.enumerate() {
        if index < skip {
            continue;
        }
        if commits.len() >= limit {
            break;
        }

        let oid = oid_result.map_err(GitError::Git)?;
        let commit = repo.find_commit(oid).map_err(GitError::Git)?;

        let author = commit.author();
        let committer = commit.committer();

        commits.push(CommitHistoryItem {
            sha: oid.to_string(),
            message: commit.message().unwrap_or("").to_string(),
            author_name: author.name().unwrap_or("").to_string(),
            author_email: author.email().unwrap_or("").to_string(),
            author_date: author.when().seconds(),
            committer_name: committer.name().unwrap_or("").to_string(),
            committer_email: committer.email().unwrap_or("").to_string(),
            committer_date: committer.when().seconds(),
            parent_count: commit.parent_count(),
        });
    }

    Ok(commits)
}

/// 获取文件差异
pub fn get_file_diff(repo_path: &str, file_path: &str, staged: bool) -> Result<String, GitError> {
    let repo = Repository::open(repo_path).map_err(GitError::Git)?;

    let mut diff_options = git2::DiffOptions::new();
    diff_options.pathspec(file_path);

    let diff = if staged {
        // 暂存区与HEAD的差异
        let head = repo.head().map_err(GitError::Git)?;
        let head_tree = head.peel_to_tree().map_err(GitError::Git)?;
        let index = repo.index().map_err(GitError::Git)?;
        repo.diff_tree_to_index(Some(&head_tree), Some(&index), Some(&mut diff_options))
            .map_err(GitError::Git)?
    } else {
        // 工作区与暂存区的差异
        repo.diff_index_to_workdir(None, Some(&mut diff_options))
            .map_err(GitError::Git)?
    };

    // 将差异转换为字符串
    let mut diff_text = String::new();
    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
        match line.origin() {
            '+' | '-' | ' ' => {
                diff_text.push(line.origin());
                diff_text.push_str(std::str::from_utf8(line.content()).unwrap_or(""));
            }
            _ => {}
        }
        true
    })
    .map_err(GitError::Git)?;

    Ok(diff_text)
}

/// 获取远程仓库信息
pub fn get_remote_info(repo_path: &str) -> Result<RemoteBranchInfo, GitError> {
    let repo = Repository::open(repo_path).map_err(GitError::Git)?;

    // 获取默认远程名称
    let remote_name = get_default_remote_name(&repo)?;

    // 获取当前分支
    let head = repo.head().map_err(GitError::Git)?;
    let branch_name = head.shorthand().unwrap_or("HEAD").to_string();

    // 获取ahead/behind状态
    let (ahead, behind) = get_ahead_behind_count(&repo)?;

    Ok(RemoteBranchInfo {
        remote_name,
        branch_name,
        ahead,
        behind,
        last_sync: None, // 这里可以从配置或其他地方获取
    })
}

/// 获取远程变更（fetch操作）
pub fn fetch_remote(repo_path: &str, remote_name: Option<&str>) -> Result<SyncResult, GitError> {
    let repo = Repository::open(repo_path).map_err(GitError::Git)?;

    // 获取远程仓库名称
    let remote_name = if let Some(name) = remote_name {
        name.to_string()
    } else {
        // 如果没有指定远程名称，尝试获取默认远程
        get_default_remote_name(&repo)?
    };

    let mut remote = repo.find_remote(&remote_name).map_err(GitError::Git)?;

    // 设置回调函数
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, _username_from_url, _allowed_types| {
        // 这里可以集成现有的认证系统
        // 暂时使用默认凭据
        git2::Cred::default()
    });

    // 设置fetch选项
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    // 执行fetch操作
    let refspecs = remote.fetch_refspecs().map_err(GitError::Git)?;
    let refspecs: Vec<&str> = refspecs.iter().flatten().collect();

    match remote.fetch(&refspecs, Some(&mut fetch_options), None) {
        Ok(()) => {
            // 获取更新后的ahead/behind状态
            let (ahead, behind) = get_ahead_behind_count(&repo)?;

            Ok(SyncResult {
                success: true,
                message: "成功获取远程变更".to_string(),
                has_conflicts: false,
                conflict_files: vec![],
                ahead,
                behind,
            })
        }
        Err(e) => Err(GitError::Git(e)),
    }
}

/// 拉取远程变更（pull操作）
pub fn pull_remote(repo_path: &str, strategy: PullStrategy) -> Result<SyncResult, GitError> {
    let repo = Repository::open(repo_path).map_err(GitError::Git)?;

    // 首先执行fetch
    let fetch_result = fetch_remote(repo_path, None)?;
    if !fetch_result.success {
        return Ok(fetch_result);
    }

    // 获取当前分支
    let head = repo.head().map_err(GitError::Git)?;
    let branch_name = head.shorthand().unwrap_or("HEAD");

    // 获取远程跟踪分支
    let upstream_name = repo
        .branch_upstream_name(&format!("refs/heads/{}", branch_name))
        .map_err(GitError::Git)?;
    let upstream_ref = repo
        .find_reference(upstream_name.as_str().unwrap())
        .map_err(GitError::Git)?;
    let upstream_commit = upstream_ref.peel_to_commit().map_err(GitError::Git)?;

    // 获取当前提交
    let local_commit = head.peel_to_commit().map_err(GitError::Git)?;

    // 检查是否需要合并
    if local_commit.id() == upstream_commit.id() {
        return Ok(SyncResult {
            success: true,
            message: "已经是最新版本".to_string(),
            has_conflicts: false,
            conflict_files: vec![],
            ahead: 0,
            behind: 0,
        });
    }

    // 执行合并或变基
    match strategy {
        PullStrategy::Merge => perform_merge(&repo, &local_commit, &upstream_commit),
        PullStrategy::Rebase => perform_rebase(&repo, &local_commit, &upstream_commit),
    }
}

/// 推送本地变更（push操作）
pub fn push_remote(
    repo_path: &str,
    remote_name: Option<&str>,
    force: bool,
) -> Result<SyncResult, GitError> {
    let repo = Repository::open(repo_path).map_err(GitError::Git)?;

    // 获取远程仓库名称
    let remote_name = if let Some(name) = remote_name {
        name.to_string()
    } else {
        // 如果没有指定远程名称，尝试获取默认远程
        get_default_remote_name(&repo)?
    };

    let mut remote = repo.find_remote(&remote_name).map_err(GitError::Git)?;

    // 获取当前分支
    let head = repo.head().map_err(GitError::Git)?;
    let branch_name = head.shorthand().unwrap_or("HEAD");

    // 设置回调函数
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, _username_from_url, _allowed_types| {
        // 这里可以集成现有的认证系统
        git2::Cred::default()
    });

    // 设置push选项
    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);

    // 构建refspec
    let refspec = if force {
        format!("+refs/heads/{}:refs/heads/{}", branch_name, branch_name)
    } else {
        format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name)
    };

    // 执行push操作
    match remote.push(&[&refspec], Some(&mut push_options)) {
        Ok(()) => {
            // 获取更新后的ahead/behind状态
            let (ahead, behind) = get_ahead_behind_count(&repo)?;

            Ok(SyncResult {
                success: true,
                message: "成功推送到远程仓库".to_string(),
                has_conflicts: false,
                conflict_files: vec![],
                ahead,
                behind,
            })
        }
        Err(e) => Err(GitError::Git(e)),
    }
}

// 辅助函数

fn is_staged(status: Status) -> bool {
    status.contains(Status::INDEX_NEW)
        || status.contains(Status::INDEX_MODIFIED)
        || status.contains(Status::INDEX_DELETED)
        || status.contains(Status::INDEX_RENAMED)
        || status.contains(Status::INDEX_TYPECHANGE)
}

fn has_unstaged_changes(status: Status) -> bool {
    status.contains(Status::WT_NEW)
        || status.contains(Status::WT_MODIFIED)
        || status.contains(Status::WT_DELETED)
        || status.contains(Status::WT_RENAMED)
        || status.contains(Status::WT_TYPECHANGE)
}

fn convert_git_status_staged(status: Status) -> String {
    if status.contains(Status::INDEX_NEW) {
        "added".to_string()
    } else if status.contains(Status::INDEX_MODIFIED) {
        "modified".to_string()
    } else if status.contains(Status::INDEX_DELETED) {
        "deleted".to_string()
    } else if status.contains(Status::INDEX_RENAMED) {
        "renamed".to_string()
    } else {
        "modified".to_string()
    }
}

fn convert_git_status_unstaged(status: Status) -> String {
    if status.contains(Status::WT_NEW) {
        "added".to_string()
    } else if status.contains(Status::WT_MODIFIED) {
        "modified".to_string()
    } else if status.contains(Status::WT_DELETED) {
        "deleted".to_string()
    } else if status.contains(Status::WT_RENAMED) {
        "renamed".to_string()
    } else {
        "modified".to_string()
    }
}

fn calculate_file_stats(
    repo: &Repository,
    file_path: &str,
    staged: bool,
) -> Result<(u32, u32), GitError> {
    let mut diff_options = git2::DiffOptions::new();
    diff_options.pathspec(file_path);

    let diff = if staged {
        // 暂存区与HEAD的差异
        let head = match repo.head() {
            Ok(head) => head,
            Err(_) => return Ok((0, 0)), // 如果没有HEAD，返回默认值
        };

        let head_tree = head.peel_to_tree().map_err(|_| GitError::Unknown {
            message: "无法获取HEAD树".to_string(),
        })?;

        let index = repo.index().map_err(GitError::Git)?;

        repo.diff_tree_to_index(Some(&head_tree), Some(&index), Some(&mut diff_options))
            .map_err(GitError::Git)?
    } else {
        // 工作区与暂存区的差异
        repo.diff_index_to_workdir(None, Some(&mut diff_options))
            .map_err(GitError::Git)?
    };

    // 计算添加和删除的行数
    let mut additions = 0;
    let mut deletions = 0;

    diff.foreach(
        &mut |_, _| true,
        None,
        None,
        Some(&mut |_, _hunk, line| {
            match line.origin() {
                '+' => additions += 1,
                '-' => deletions += 1,
                _ => {}
            }
            true
        }),
    )
    .map_err(GitError::Git)?;

    Ok((additions, deletions))
}

fn get_current_branch(repo: &Repository) -> Result<String, GitError> {
    match repo.head() {
        Ok(head) => {
            if let Some(branch_name) = head.shorthand() {
                Ok(branch_name.to_string())
            } else {
                Ok("HEAD".to_string())
            }
        }
        Err(_) => Ok("main".to_string()),
    }
}

fn get_ahead_behind_count(repo: &Repository) -> Result<(u32, u32), GitError> {
    // 获取当前分支
    let head = match repo.head() {
        Ok(head) => head,
        Err(_) => return Ok((0, 0)), // 如果没有HEAD，返回默认值
    };

    // 获取当前分支名
    let branch_name = match head.shorthand() {
        Some(name) => name,
        None => return Ok((0, 0)), // 如果无法获取分支名，返回默认值
    };

    // 查找对应的远程跟踪分支
    let upstream_name = match repo.branch_upstream_name(&format!("refs/heads/{}", branch_name)) {
        Ok(name) => name,
        Err(_) => return Ok((0, 0)), // 如果没有上游分支，返回默认值
    };

    // 获取本地和远程分支的OID
    let local_oid = head.target().ok_or_else(|| GitError::Unknown {
        message: "无法获取本地分支OID".to_string(),
    })?;

    let upstream_ref = repo
        .find_reference(upstream_name.as_str().unwrap())
        .map_err(|_| GitError::Unknown {
            message: "无法找到远程跟踪分支".to_string(),
        })?;

    let upstream_oid = upstream_ref.target().ok_or_else(|| GitError::Unknown {
        message: "无法获取远程分支OID".to_string(),
    })?;

    // 计算领先/落后数量
    let (ahead, behind) = repo
        .graph_ahead_behind(local_oid, upstream_oid)
        .map_err(GitError::Git)?;

    Ok((ahead as u32, behind as u32))
}

/// 获取默认远程名称
pub fn get_default_remote_name(repo: &Repository) -> Result<String, GitError> {
    log::debug!("开始检测默认远程名称");

    // 首先尝试获取当前分支的上游远程
    if let Ok(head) = repo.head() {
        if let Some(branch_name) = head.shorthand() {
            log::debug!("当前分支: {}", branch_name);

            if let Ok(upstream_name) =
                repo.branch_upstream_name(&format!("refs/heads/{}", branch_name))
            {
                if let Some(upstream_str) = upstream_name.as_str() {
                    log::debug!("找到上游分支: {}", upstream_str);

                    // 解析远程名称，格式通常是 "refs/remotes/origin/main"
                    if let Some(remote_part) = upstream_str.strip_prefix("refs/remotes/") {
                        if let Some(slash_pos) = remote_part.find('/') {
                            let remote_name = remote_part[..slash_pos].to_string();
                            log::debug!("从上游分支解析到远程名称: {}", remote_name);
                            return Ok(remote_name);
                        }
                    }
                }
            } else {
                log::debug!("当前分支没有设置上游分支");
            }
        }
    }

    // 如果无法从分支获取，尝试列出所有远程
    let remotes = repo.remotes().map_err(GitError::Git)?;
    let remote_names: Vec<String> = remotes.iter().flatten().map(|s| s.to_string()).collect();
    log::debug!("仓库中的远程列表: {:?}", remote_names);

    if remotes.len() == 0 {
        log::error!("仓库中没有配置任何远程仓库");
        return Err(GitError::Unknown {
            message: "仓库中没有配置任何远程仓库".to_string(),
        });
    }

    // 优先选择常见的远程名称
    let preferred_remotes = ["origin", "upstream", "github", "gitlab"];
    for preferred in &preferred_remotes {
        for i in 0..remotes.len() {
            if let Some(remote_name) = remotes.get(i) {
                if remote_name == *preferred {
                    log::debug!("找到首选远程名称: {}", remote_name);
                    return Ok(remote_name.to_string());
                }
            }
        }
    }

    // 如果没有找到首选的，使用第一个可用的远程
    if let Some(first_remote) = remotes.get(0) {
        log::debug!("使用第一个可用的远程: {}", first_remote);
        return Ok(first_remote.to_string());
    }

    // 这种情况理论上不应该发生，因为我们已经检查了remotes.len() == 0
    Err(GitError::Unknown {
        message: "无法确定默认远程名称".to_string(),
    })
}

/// 执行合并操作
fn perform_merge(
    repo: &Repository,
    local_commit: &git2::Commit,
    upstream_commit: &git2::Commit,
) -> Result<SyncResult, GitError> {
    // 获取合并基础
    let merge_base = repo
        .merge_base(local_commit.id(), upstream_commit.id())
        .map_err(GitError::Git)?;
    let merge_base_commit = repo.find_commit(merge_base).map_err(GitError::Git)?;

    // 创建AnnotatedCommit用于合并分析
    let upstream_annotated = repo
        .find_annotated_commit(upstream_commit.id())
        .map_err(GitError::Git)?;
    let analysis = repo
        .merge_analysis(&[&upstream_annotated])
        .map_err(GitError::Git)?;

    if analysis.0.is_fast_forward() {
        // 快进合并
        let refname = format!(
            "refs/heads/{}",
            repo.head()
                .map_err(GitError::Git)?
                .shorthand()
                .unwrap_or("HEAD")
        );
        let mut reference = repo.find_reference(&refname).map_err(GitError::Git)?;
        reference
            .set_target(upstream_commit.id(), "Fast-forward merge")
            .map_err(GitError::Git)?;

        // 更新工作目录
        repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
            .map_err(GitError::Git)?;

        Ok(SyncResult {
            success: true,
            message: "快进合并成功".to_string(),
            has_conflicts: false,
            conflict_files: vec![],
            ahead: 0,
            behind: 0,
        })
    } else if analysis.0.is_normal() {
        // 正常合并
        let local_tree = local_commit.tree().map_err(GitError::Git)?;
        let upstream_tree = upstream_commit.tree().map_err(GitError::Git)?;
        let ancestor_tree = merge_base_commit.tree().map_err(GitError::Git)?;

        let mut index = repo
            .merge_trees(&ancestor_tree, &local_tree, &upstream_tree, None)
            .map_err(GitError::Git)?;

        if index.has_conflicts() {
            // 有冲突，需要用户解决
            let conflict_files: Vec<String> = index
                .conflicts()
                .map_err(GitError::Git)?
                .filter_map(|conflict| {
                    conflict.ok().and_then(|c| {
                        c.our.as_ref().and_then(|entry| {
                            std::str::from_utf8(&entry.path).ok().map(|s| s.to_string())
                        })
                    })
                })
                .collect();

            return Ok(SyncResult {
                success: false,
                message: "合并时发现冲突，请手动解决".to_string(),
                has_conflicts: true,
                conflict_files,
                ahead: 0,
                behind: 0,
            });
        }

        // 无冲突，创建合并提交
        let signature = repo.signature().map_err(GitError::Git)?;
        let tree_id = index.write_tree_to(repo).map_err(GitError::Git)?;
        let tree = repo.find_tree(tree_id).map_err(GitError::Git)?;

        let message = format!(
            "Merge branch '{}'",
            upstream_commit.summary().unwrap_or("unknown")
        );

        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            &message,
            &tree,
            &[local_commit, upstream_commit],
        )
        .map_err(GitError::Git)?;

        Ok(SyncResult {
            success: true,
            message: "合并成功".to_string(),
            has_conflicts: false,
            conflict_files: vec![],
            ahead: 0,
            behind: 0,
        })
    } else {
        Ok(SyncResult {
            success: true,
            message: "无需合并".to_string(),
            has_conflicts: false,
            conflict_files: vec![],
            ahead: 0,
            behind: 0,
        })
    }
}

/// 执行变基操作
fn perform_rebase(
    repo: &Repository,
    local_commit: &git2::Commit,
    upstream_commit: &git2::Commit,
) -> Result<SyncResult, GitError> {
    // 变基操作比较复杂，这里提供一个简化的实现
    // 在实际项目中，可能需要更复杂的冲突处理逻辑

    let signature = repo.signature().map_err(GitError::Git)?;

    // 创建AnnotatedCommit用于变基
    let local_annotated = repo
        .find_annotated_commit(local_commit.id())
        .map_err(GitError::Git)?;
    let upstream_annotated = repo
        .find_annotated_commit(upstream_commit.id())
        .map_err(GitError::Git)?;

    // 创建变基操作
    let mut rebase = repo
        .rebase(
            Some(&local_annotated),
            Some(&upstream_annotated),
            None,
            None,
        )
        .map_err(GitError::Git)?;

    // 执行变基步骤
    while let Some(operation) = rebase.next() {
        match operation {
            Ok(_op) => {
                // 检查是否有冲突
                let index = repo.index().map_err(GitError::Git)?;
                if index.has_conflicts() {
                    // 有冲突，中止变基
                    rebase.abort().map_err(GitError::Git)?;

                    let conflict_files: Vec<String> = index
                        .conflicts()
                        .map_err(GitError::Git)?
                        .filter_map(|conflict| {
                            conflict.ok().and_then(|c| {
                                c.our.as_ref().and_then(|entry| {
                                    std::str::from_utf8(&entry.path).ok().map(|s| s.to_string())
                                })
                            })
                        })
                        .collect();

                    return Ok(SyncResult {
                        success: false,
                        message: "变基时发现冲突，请手动解决".to_string(),
                        has_conflicts: true,
                        conflict_files,
                        ahead: 0,
                        behind: 0,
                    });
                }

                // 提交当前步骤
                rebase
                    .commit(None, &signature, None)
                    .map_err(GitError::Git)?;
            }
            Err(e) => {
                rebase.abort().map_err(GitError::Git)?;
                return Err(GitError::Git(e));
            }
        }
    }

    // 完成变基
    rebase.finish(None).map_err(GitError::Git)?;

    Ok(SyncResult {
        success: true,
        message: "变基成功".to_string(),
        has_conflicts: false,
        conflict_files: vec![],
        ahead: 0,
        behind: 0,
    })
}

// ==================== Token认证系统 ====================

/// Token配置结构
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TokenConfig {
    pub domain: String,
    pub token: String,
    pub username: Option<String>,
    pub created_at: i64,
    pub last_used: Option<i64>,
}

/// 创建支持Token认证的回调函数
pub fn create_authenticated_callbacks(
    repo_url: &str,
    token_cache: Option<String>,
) -> RemoteCallbacks {
    let mut callbacks = RemoteCallbacks::new();
    let _url = repo_url.to_string();

    // 添加认证尝试计数器，防止无限重试
    let auth_attempts = std::cell::RefCell::new(0);

    callbacks.credentials(move |url, username_from_url, allowed_types| {
        let mut attempts = auth_attempts.borrow_mut();
        *attempts += 1;

        log::debug!(
            "Git认证回调被调用: url={}, username={:?}, allowed_types={:?}, 尝试次数={}",
            url,
            username_from_url,
            allowed_types,
            *attempts
        );

        // 防止无限重试，最多尝试3次
        if *attempts > 3 {
            log::error!("认证尝试次数过多，停止重试");
            return Err(git2::Error::from_str("认证失败：尝试次数过多"));
        }

        // 检查是否是HTTPS协议
        if url.starts_with("https://") || url.starts_with("http://") {
            // 尝试使用Token认证
            if let Some(ref token) = token_cache {
                log::debug!("使用Token进行HTTPS认证 (尝试 {})", *attempts);

                // 对于AtomGit等服务，Token应该作为用户名，密码为空或token
                // 第一次尝试：token作为用户名，密码为空
                if *attempts == 1 {
                    return git2::Cred::userpass_plaintext(token, "");
                }
                // 第二次尝试：使用用户名和token作为密码
                else if *attempts == 2 {
                    let username = username_from_url.unwrap_or("git");
                    return git2::Cred::userpass_plaintext(username, token);
                }
            } else {
                log::debug!("未找到Token，尝试默认凭据");
                // 只在第一次尝试时使用默认凭据
                if *attempts == 1 {
                    return git2::Cred::default();
                }
            }
        }

        // 对于SSH或其他情况，使用默认凭据
        if *attempts == 1 {
            log::debug!("使用默认凭据");
            git2::Cred::default()
        } else {
            log::error!("认证失败，无更多认证方式可尝试");
            Err(git2::Error::from_str("认证失败"))
        }
    });

    // 添加证书检查回调
    callbacks.certificate_check(|_cert, _valid| {
        log::debug!("证书检查回调");
        // 对于开发环境，可以选择接受所有证书
        // 生产环境应该进行适当的证书验证
        Ok(git2::CertificateCheckStatus::CertificateOk)
    });

    callbacks
}

// ==================== 分支管理功能 ====================

/// 分支信息结构
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct BranchInfo {
    /// 分支名称
    pub name: String,
    /// 是否为当前分支
    pub is_current: bool,
    /// 是否为远程分支
    pub is_remote: bool,
    /// 上游分支名称
    pub upstream: Option<String>,
    /// 领先提交数
    pub ahead: u32,
    /// 落后提交数
    pub behind: u32,
    /// 最后提交信息
    pub last_commit: Option<CommitInfo>,
}

/// 简化的提交信息结构
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CommitInfo {
    /// 提交SHA
    pub sha: String,
    /// 提交消息
    pub message: String,
    /// 作者姓名
    pub author_name: String,
    /// 作者邮箱
    pub author_email: String,
    /// 提交时间戳
    pub timestamp: i64,
}

/// 分支切换结果
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SwitchResult {
    /// 操作是否成功
    pub success: bool,
    /// 操作消息
    pub message: String,
    /// 是否有未提交的变更
    pub has_uncommitted_changes: bool,
    /// 未提交的文件列表
    pub uncommitted_files: Vec<String>,
}

/// 获取分支列表
pub fn list_branches(repo_path: &str) -> Result<Vec<BranchInfo>, GitError> {
    let repo = Repository::open(repo_path).map_err(GitError::Git)?;
    let mut branches = Vec::new();

    log::debug!("开始获取分支列表: {}", repo_path);

    // 获取当前分支名称
    let current_branch_name = get_current_branch_name(&repo)?;
    log::debug!("当前分支: {:?}", current_branch_name);

    // 获取本地分支
    let local_branches = repo
        .branches(Some(git2::BranchType::Local))
        .map_err(GitError::Git)?;
    for branch_result in local_branches {
        if let Ok((branch, _)) = branch_result {
            if let Some(name) = branch.name().map_err(GitError::Git)? {
                let is_current = current_branch_name.as_ref() == Some(&name.to_string());

                log::debug!("处理本地分支: {} (当前: {})", name, is_current);

                let branch_info =
                    create_branch_info(&repo, name.to_string(), is_current, false, &branch)?;

                branches.push(branch_info);
            }
        }
    }

    // 获取远程分支
    let remote_branches = repo
        .branches(Some(git2::BranchType::Remote))
        .map_err(GitError::Git)?;
    for branch_result in remote_branches {
        if let Ok((branch, _)) = branch_result {
            if let Some(name) = branch.name().map_err(GitError::Git)? {
                // 过滤掉远程HEAD分支
                if is_remote_head_branch(name) {
                    log::debug!("跳过远程HEAD分支: {}", name);
                    continue;
                }

                log::debug!("处理远程分支: {}", name);

                let branch_info =
                    create_branch_info(&repo, name.to_string(), false, true, &branch)?;

                branches.push(branch_info);
            }
        }
    }

    log::debug!("获取到 {} 个分支", branches.len());
    Ok(branches)
}

/// 创建分支信息
fn create_branch_info(
    repo: &Repository,
    name: String,
    is_current: bool,
    is_remote: bool,
    branch: &git2::Branch,
) -> Result<BranchInfo, GitError> {
    // 获取上游分支
    let upstream = if !is_remote {
        branch.upstream().ok().and_then(|upstream_branch| {
            upstream_branch.name().ok().flatten().map(|s| s.to_string())
        })
    } else {
        None
    };

    // 获取分支的最后提交
    let last_commit = {
        let reference = branch.get();
        if let Some(target_oid) = reference.target() {
            if let Ok(commit) = repo.find_commit(target_oid) {
                Some(CommitInfo {
                    sha: commit.id().to_string(),
                    message: commit.message().unwrap_or("").to_string(),
                    author_name: commit.author().name().unwrap_or("").to_string(),
                    author_email: commit.author().email().unwrap_or("").to_string(),
                    timestamp: commit.time().seconds(),
                })
            } else {
                None
            }
        } else {
            None
        }
    };

    // 计算ahead/behind状态（仅对有上游的本地分支）
    let (ahead, behind) = if !is_remote && upstream.is_some() {
        calculate_ahead_behind_for_branch(repo, &name).unwrap_or((0, 0))
    } else {
        (0, 0)
    };

    Ok(BranchInfo {
        name,
        is_current,
        is_remote,
        upstream,
        ahead,
        behind,
        last_commit,
    })
}

/// 获取当前分支名称
fn get_current_branch_name(repo: &Repository) -> Result<Option<String>, GitError> {
    match repo.head() {
        Ok(head) => {
            if let Some(shorthand) = head.shorthand() {
                Ok(Some(shorthand.to_string()))
            } else {
                Ok(None)
            }
        }
        Err(_) => Ok(None), // 可能是新仓库或detached HEAD
    }
}

/// 检查是否是远程HEAD分支
fn is_remote_head_branch(branch_name: &str) -> bool {
    // 远程HEAD分支通常以 "/HEAD" 结尾
    // 例如: "origin/HEAD", "upstream/HEAD" 等
    branch_name.ends_with("/HEAD")
}

/// 计算特定分支的ahead/behind状态
fn calculate_ahead_behind_for_branch(
    repo: &Repository,
    branch_name: &str,
) -> Result<(u32, u32), GitError> {
    // 尝试获取分支的上游
    let upstream_name = match repo.branch_upstream_name(&format!("refs/heads/{}", branch_name)) {
        Ok(name) => name,
        Err(_) => return Ok((0, 0)), // 没有上游分支
    };

    if let Some(upstream_str) = upstream_name.as_str() {
        // 获取本地分支和上游分支的提交
        let local_ref = format!("refs/heads/{}", branch_name);
        let upstream_ref = upstream_str;

        if let (Ok(local_oid), Ok(upstream_oid)) = (
            repo.refname_to_id(&local_ref),
            repo.refname_to_id(upstream_ref),
        ) {
            return calculate_ahead_behind_from_oids(repo, local_oid, upstream_oid);
        }
    }

    Ok((0, 0))
}

/// 从OID计算ahead/behind
fn calculate_ahead_behind_from_oids(
    repo: &Repository,
    local_oid: git2::Oid,
    upstream_oid: git2::Oid,
) -> Result<(u32, u32), GitError> {
    match repo.graph_ahead_behind(local_oid, upstream_oid) {
        Ok((ahead, behind)) => Ok((ahead as u32, behind as u32)),
        Err(_) => Ok((0, 0)),
    }
}

/// 支持Token认证的fetch操作
pub fn fetch_remote_with_token(
    repo_path: &str,
    remote_name: Option<&str>,
    token_cache: Option<String>,
) -> Result<SyncResult, GitError> {
    let repo = Repository::open(repo_path).map_err(GitError::Git)?;

    // 获取远程仓库名称
    let remote_name = if let Some(name) = remote_name {
        name.to_string()
    } else {
        // 如果没有指定远程名称，尝试获取默认远程
        get_default_remote_name(&repo)?
    };

    let mut remote = repo.find_remote(&remote_name).map_err(GitError::Git)?;

    // 获取远程URL
    let remote_url = remote.url().unwrap_or("").to_string();
    log::debug!("Fetch操作使用远程URL: {}", remote_url);

    // 创建支持Token认证的回调
    let callbacks = create_authenticated_callbacks(&remote_url, token_cache);

    // 设置fetch选项
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    // 执行fetch操作
    let refspecs = remote.fetch_refspecs().map_err(GitError::Git)?;
    let refspecs: Vec<&str> = refspecs.iter().flatten().collect();

    match remote.fetch(&refspecs, Some(&mut fetch_options), None) {
        Ok(()) => {
            // 获取更新后的ahead/behind状态
            let (ahead, behind) = get_ahead_behind_count(&repo)?;

            Ok(SyncResult {
                success: true,
                message: "成功获取远程变更".to_string(),
                has_conflicts: false,
                conflict_files: vec![],
                ahead,
                behind,
            })
        }
        Err(e) => {
            log::error!("Fetch操作失败: {}", e);
            Err(GitError::Git(e))
        }
    }
}

/// 支持Token认证的push操作
pub fn push_remote_with_token(
    repo_path: &str,
    remote_name: Option<&str>,
    force: bool,
    token_cache: Option<String>,
) -> Result<SyncResult, GitError> {
    let repo = Repository::open(repo_path).map_err(GitError::Git)?;

    // 获取远程仓库名称
    let remote_name = if let Some(name) = remote_name {
        name.to_string()
    } else {
        // 如果没有指定远程名称，尝试获取默认远程
        get_default_remote_name(&repo)?
    };

    let mut remote = repo.find_remote(&remote_name).map_err(GitError::Git)?;

    // 获取当前分支
    let head = repo.head().map_err(GitError::Git)?;
    let branch_name = head.shorthand().unwrap_or("HEAD");

    // 获取远程URL
    let remote_url = remote.url().unwrap_or("").to_string();
    log::debug!("Push操作使用远程URL: {}", remote_url);

    // 创建支持Token认证的回调
    let callbacks = create_authenticated_callbacks(&remote_url, token_cache);

    // 设置push选项
    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);

    // 构建refspec
    let refspec = if force {
        format!("+refs/heads/{}:refs/heads/{}", branch_name, branch_name)
    } else {
        format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name)
    };

    // 执行push操作
    match remote.push(&[&refspec], Some(&mut push_options)) {
        Ok(()) => {
            // 获取更新后的ahead/behind状态
            let (ahead, behind) = get_ahead_behind_count(&repo)?;

            Ok(SyncResult {
                success: true,
                message: "成功推送到远程仓库".to_string(),
                has_conflicts: false,
                conflict_files: vec![],
                ahead,
                behind,
            })
        }
        Err(e) => {
            log::error!("Push操作失败: {}", e);
            Err(GitError::Git(e))
        }
    }
}

/// 创建新分支
pub fn create_branch(
    repo_path: &str,
    branch_name: &str,
    from_commit: Option<&str>,
    checkout: bool,
) -> Result<SwitchResult, GitError> {
    let repo = Repository::open(repo_path).map_err(GitError::Git)?;

    log::debug!(
        "创建分支: {} (from: {:?}, checkout: {})",
        branch_name,
        from_commit,
        checkout
    );

    // 检查分支名称是否已存在
    if let Ok(_) = repo.find_branch(branch_name, git2::BranchType::Local) {
        return Err(GitError::Unknown {
            message: format!("分支 '{}' 已存在", branch_name),
        });
    }

    // 获取目标提交
    let target_commit = if let Some(commit_ref) = from_commit {
        log::debug!("从指定引用创建分支: {}", commit_ref);

        // 首先尝试作为分支名称解析
        if let Ok(branch) = repo.find_branch(commit_ref, git2::BranchType::Local) {
            let branch_ref = branch.get();
            let oid = branch_ref.target().ok_or_else(|| GitError::Unknown {
                message: format!("无法获取分支 '{}' 的目标", commit_ref),
            })?;
            repo.find_commit(oid).map_err(GitError::Git)?
        }
        // 然后尝试作为提交SHA解析
        else if let Ok(oid) = git2::Oid::from_str(commit_ref) {
            repo.find_commit(oid).map_err(GitError::Git)?
        }
        // 最后尝试作为引用名称解析
        else if let Ok(reference) = repo.find_reference(commit_ref) {
            let oid = reference.target().ok_or_else(|| GitError::Unknown {
                message: format!("无法获取引用 '{}' 的目标", commit_ref),
            })?;
            repo.find_commit(oid).map_err(GitError::Git)?
        }
        // 尝试作为完整的引用路径解析
        else if let Ok(reference) = repo.find_reference(&format!("refs/heads/{}", commit_ref)) {
            let oid = reference.target().ok_or_else(|| GitError::Unknown {
                message: format!("无法获取引用 'refs/heads/{}' 的目标", commit_ref),
            })?;
            repo.find_commit(oid).map_err(GitError::Git)?
        } else {
            return Err(GitError::Unknown {
                message: format!("无法找到分支、提交或引用: '{}'", commit_ref),
            });
        }
    } else {
        // 从当前HEAD创建
        log::debug!("从当前HEAD创建分支");
        let head = repo.head().map_err(GitError::Git)?;
        let oid = head.target().ok_or_else(|| GitError::Unknown {
            message: "无法获取HEAD目标".to_string(),
        })?;
        repo.find_commit(oid).map_err(GitError::Git)?
    };

    // 创建分支
    let _branch = repo
        .branch(branch_name, &target_commit, false)
        .map_err(GitError::Git)?;
    log::debug!("分支创建成功: {}", branch_name);

    // 如果需要切换到新分支
    if checkout {
        return switch_branch(repo_path, branch_name);
    }

    Ok(SwitchResult {
        success: true,
        message: format!("成功创建分支 '{}'", branch_name),
        has_uncommitted_changes: false,
        uncommitted_files: vec![],
    })
}

/// 切换分支
pub fn switch_branch(repo_path: &str, branch_name: &str) -> Result<SwitchResult, GitError> {
    let repo = Repository::open(repo_path).map_err(GitError::Git)?;

    log::debug!("切换分支: {}", branch_name);

    // 检查是否有未提交的变更
    let status = get_repository_status(repo_path)?;
    let has_uncommitted = !status.files.is_empty();
    let uncommitted_files: Vec<String> = status.files.iter().map(|f| f.path.clone()).collect();

    if has_uncommitted {
        return Ok(SwitchResult {
            success: false,
            message: "存在未提交的变更，请先提交或暂存变更".to_string(),
            has_uncommitted_changes: true,
            uncommitted_files,
        });
    }

    // 查找目标分支
    let branch = repo
        .find_branch(branch_name, git2::BranchType::Local)
        .map_err(|_| GitError::Unknown {
            message: format!("分支 '{}' 不存在", branch_name),
        })?;

    // 获取分支的引用
    let branch_ref = branch.get();
    let branch_oid = branch_ref.target().ok_or_else(|| GitError::Unknown {
        message: "无法获取分支目标".to_string(),
    })?;

    // 设置HEAD到目标分支
    let refname = format!("refs/heads/{}", branch_name);
    repo.set_head(&refname).map_err(GitError::Git)?;

    // 检出工作目录
    let commit = repo.find_commit(branch_oid).map_err(GitError::Git)?;
    let tree = commit.tree().map_err(GitError::Git)?;

    repo.checkout_tree(
        tree.as_object(),
        Some(
            git2::build::CheckoutBuilder::new()
                .safe()
                .recreate_missing(true),
        ),
    )
    .map_err(GitError::Git)?;

    log::debug!("分支切换成功: {}", branch_name);

    Ok(SwitchResult {
        success: true,
        message: format!("成功切换到分支 '{}'", branch_name),
        has_uncommitted_changes: false,
        uncommitted_files: vec![],
    })
}

/// 删除分支
pub fn delete_branch(
    repo_path: &str,
    branch_name: &str,
    force: bool,
) -> Result<SwitchResult, GitError> {
    let repo = Repository::open(repo_path).map_err(GitError::Git)?;

    log::debug!("删除分支: {} (force: {})", branch_name, force);

    // 检查是否是当前分支
    if let Ok(current_name) = get_current_branch_name(&repo) {
        if current_name.as_ref() == Some(&branch_name.to_string()) {
            return Err(GitError::Unknown {
                message: "不能删除当前分支".to_string(),
            });
        }
    }

    // 查找分支
    let mut branch = repo
        .find_branch(branch_name, git2::BranchType::Local)
        .map_err(|_| GitError::Unknown {
            message: format!("分支 '{}' 不存在", branch_name),
        })?;

    // 如果不是强制删除，检查分支是否已合并
    if !force {
        // 检查分支是否有未合并的提交
        if let Ok(upstream) = branch.upstream() {
            let branch_oid = branch.get().target().unwrap();
            let upstream_oid = upstream.get().target().unwrap();

            if let Ok((ahead, _)) = repo.graph_ahead_behind(branch_oid, upstream_oid) {
                if ahead > 0 {
                    return Err(GitError::Unknown {
                        message: format!(
                            "分支 '{}' 包含未合并的提交，使用强制删除或先合并分支",
                            branch_name
                        ),
                    });
                }
            }
        }
    }

    // 删除分支
    branch.delete().map_err(GitError::Git)?;
    log::debug!("分支删除成功: {}", branch_name);

    Ok(SwitchResult {
        success: true,
        message: format!("成功删除分支 '{}'", branch_name),
        has_uncommitted_changes: false,
        uncommitted_files: vec![],
    })
}

/// 从远程分支检出本地分支
pub fn checkout_remote_branch(
    repo_path: &str,
    remote_branch_name: &str,
    local_branch_name: Option<&str>,
) -> Result<SwitchResult, GitError> {
    let repo = Repository::open(repo_path).map_err(GitError::Git)?;

    log::debug!(
        "检出远程分支: {} -> {:?}",
        remote_branch_name,
        local_branch_name
    );

    // 提取本地分支名称
    let local_name = if let Some(name) = local_branch_name {
        name.to_string()
    } else {
        extract_local_branch_name(remote_branch_name)
    };

    log::debug!("本地分支名称: {}", local_name);

    // 检查本地分支是否已存在
    if let Ok(_) = repo.find_branch(&local_name, git2::BranchType::Local) {
        return Err(GitError::Unknown {
            message: format!("本地分支 '{}' 已存在", local_name),
        });
    }

    // 查找远程分支
    let remote_branch = repo
        .find_branch(remote_branch_name, git2::BranchType::Remote)
        .map_err(|_| GitError::Unknown {
            message: format!("远程分支 '{}' 不存在", remote_branch_name),
        })?;

    // 获取远程分支的提交
    let remote_ref = remote_branch.get();
    let remote_oid = remote_ref.target().ok_or_else(|| GitError::Unknown {
        message: format!("无法获取远程分支 '{}' 的目标", remote_branch_name),
    })?;
    let remote_commit = repo.find_commit(remote_oid).map_err(GitError::Git)?;

    // 创建本地分支
    let local_branch = repo
        .branch(&local_name, &remote_commit, false)
        .map_err(GitError::Git)?;
    log::debug!("本地分支创建成功: {}", local_name);

    // 设置上游跟踪分支
    if let Err(e) = set_upstream_branch(&repo, &local_name, remote_branch_name) {
        log::warn!("设置上游分支失败: {}", e);
        // 不影响主要功能，继续执行
    }

    // 切换到新创建的本地分支
    let switch_result = switch_branch(repo_path, &local_name)?;

    if switch_result.success {
        Ok(SwitchResult {
            success: true,
            message: format!(
                "成功检出远程分支 '{}' 到本地分支 '{}'",
                remote_branch_name, local_name
            ),
            has_uncommitted_changes: false,
            uncommitted_files: vec![],
        })
    } else {
        // 如果切换失败，删除刚创建的分支
        let mut branch_to_delete = local_branch;
        let _ = branch_to_delete.delete();
        Err(GitError::Unknown {
            message: format!("检出成功但切换失败: {}", switch_result.message),
        })
    }
}

/// 从远程分支名称提取本地分支名称
fn extract_local_branch_name(remote_branch_name: &str) -> String {
    // "origin/feature/new-ui" -> "feature/new-ui"
    if let Some(pos) = remote_branch_name.find('/') {
        remote_branch_name[pos + 1..].to_string()
    } else {
        remote_branch_name.to_string()
    }
}

/// 设置上游跟踪分支
fn set_upstream_branch(
    repo: &Repository,
    local_branch: &str,
    remote_branch: &str,
) -> Result<(), GitError> {
    log::debug!("设置上游分支: {} -> {}", local_branch, remote_branch);

    // 解析远程名称和分支名称
    let (remote_name, branch_name) = if let Some(pos) = remote_branch.find('/') {
        let remote = &remote_branch[..pos];
        let branch = &remote_branch[pos + 1..];
        (remote, branch)
    } else {
        return Err(GitError::Unknown {
            message: "无效的远程分支名称格式".to_string(),
        });
    };

    // 设置配置
    let mut config = repo.config().map_err(GitError::Git)?;

    // 设置远程名称
    config
        .set_str(&format!("branch.{}.remote", local_branch), remote_name)
        .map_err(GitError::Git)?;

    // 设置合并引用
    config
        .set_str(
            &format!("branch.{}.merge", local_branch),
            &format!("refs/heads/{}", branch_name),
        )
        .map_err(GitError::Git)?;

    log::debug!(
        "上游分支设置成功: {} -> {}/{}",
        local_branch,
        remote_name,
        branch_name
    );
    Ok(())
}
