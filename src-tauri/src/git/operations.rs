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
fn get_default_remote_name(repo: &Repository) -> Result<String, GitError> {
    // 首先尝试获取当前分支的上游远程
    if let Ok(head) = repo.head() {
        if let Some(branch_name) = head.shorthand() {
            if let Ok(upstream_name) =
                repo.branch_upstream_name(&format!("refs/heads/{}", branch_name))
            {
                if let Some(upstream_str) = upstream_name.as_str() {
                    // 解析远程名称，格式通常是 "refs/remotes/origin/main"
                    if let Some(remote_part) = upstream_str.strip_prefix("refs/remotes/") {
                        if let Some(slash_pos) = remote_part.find('/') {
                            return Ok(remote_part[..slash_pos].to_string());
                        }
                    }
                }
            }
        }
    }

    // 如果无法从分支获取，尝试列出所有远程并选择第一个
    let remotes = repo.remotes().map_err(GitError::Git)?;
    if let Some(first_remote) = remotes.get(0) {
        return Ok(first_remote.to_string());
    }

    // 最后回退到 "origin"
    Ok("origin".to_string())
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
