use crate::git::types::{CommitHistoryItem, CommitOptions, FileStatus, GitError, RepositoryStatus};
use git2::{Repository, Signature, Status, StatusOptions};
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

        // 转换Git状态为我们的FileStatus
        let status = convert_git_status(git_status);
        let staged = is_staged(git_status);

        // 计算文件变更统计（简化版本）
        let (additions, deletions) = calculate_file_stats(&repo, &path, staged)?;

        files.push(FileStatus {
            path,
            status,
            staged,
            additions,
            deletions,
        });
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

// 辅助函数

fn convert_git_status(status: Status) -> String {
    if status.contains(Status::WT_NEW) || status.contains(Status::INDEX_NEW) {
        "added".to_string()
    } else if status.contains(Status::WT_MODIFIED) || status.contains(Status::INDEX_MODIFIED) {
        "modified".to_string()
    } else if status.contains(Status::WT_DELETED) || status.contains(Status::INDEX_DELETED) {
        "deleted".to_string()
    } else if status.contains(Status::WT_RENAMED) || status.contains(Status::INDEX_RENAMED) {
        "renamed".to_string()
    } else {
        "untracked".to_string()
    }
}

fn is_staged(status: Status) -> bool {
    status.contains(Status::INDEX_NEW)
        || status.contains(Status::INDEX_MODIFIED)
        || status.contains(Status::INDEX_DELETED)
        || status.contains(Status::INDEX_RENAMED)
        || status.contains(Status::INDEX_TYPECHANGE)
}

fn calculate_file_stats(
    _repo: &Repository,
    _file_path: &str,
    _staged: bool,
) -> Result<(u32, u32), GitError> {
    // 简化版本：返回默认值
    // 实际实现需要计算具体的行数变更
    Ok((0, 0))
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

fn get_ahead_behind_count(_repo: &Repository) -> Result<(u32, u32), GitError> {
    // 简化版本：返回默认值
    // 实际实现需要比较本地分支与远程分支
    Ok((0, 0))
}
