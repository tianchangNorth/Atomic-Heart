// Git 相关类型定义

export interface GitCommit {
  sha: string;
  url: string;
  html_url?: string;
  message: string;
  author: GitUser;
  committer: GitUser;
  date?: string;
  tree?: {
    sha: string;
    url: string;
  };
  parents?: Array<{
    sha: string;
    url: string;
    html_url: string;
  }>;
  verification?: {
    verified: boolean;
    reason: string;
    signature?: string;
    payload?: string;
  };
  stats?: {
    additions: number;
    deletions: number;
    total: number;
  };
  files?: GitFile[];
}

export interface GitUser {
  name: string;
  email: string;
  date: string;
  avatar_url?: string;
  login?: string;
}

export interface GitFile {
  sha?: string;
  filename: string;
  status: 'added' | 'removed' | 'modified' | 'renamed' | 'copied' | 'changed' | 'unchanged';
  additions: number;
  deletions: number;
  changes: number;
  blob_url: string;
  raw_url: string;
  contents_url: string;
  patch?: string;
  previous_filename?: string;
}

export interface GitTree {
  sha: string;
  url: string;
  tree: GitTreeItem[];
  truncated: boolean;
}

export interface GitTreeItem {
  path: string;
  mode: string;
  type: 'blob' | 'tree' | 'commit';
  sha: string;
  size?: number;
  url: string;
}

export interface GitBlob {
  sha: string;
  url: string;
  content: string;
  encoding: 'base64' | 'utf-8';
  size: number;
}

export interface GitRef {
  ref: string;
  node_id: string;
  url: string;
  object: {
    sha: string;
    type: 'commit' | 'tree' | 'blob' | 'tag';
    url: string;
  };
}

export interface GitTag {
  sha: string;
  url: string;
  tagger: GitUser;
  object: {
    sha: string;
    type: 'commit' | 'tree' | 'blob' | 'tag';
    url: string;
  };
  tag: string;
  message: string;
}

// Git 操作相关类型
export interface CloneOptions {
  url: string;
  directory?: string;
  branch?: string;
  depth?: string | number;
  recursive?: boolean;
  auth?: {
    type: 'password' | 'ssh' | 'token';
    username?: string;
    password?: string;
    token?: string;
    sshKeyPath?: string;
  };
}

// 本地仓库管理相关类型
export interface LocalRepository {
  id: string;
  name: string;
  path: string;
  remoteUrl?: string;
  currentBranch: string;
  status: 'clean' | 'dirty' | 'conflict' | 'syncing';
  lastCommit?: GitCommit;
  ahead: number;
  behind: number;
  createdAt: string;
  updatedAt: string;
}

export interface FileChange {
  path: string;
  status: 'added' | 'modified' | 'deleted' | 'renamed' | 'untracked';
  staged: boolean;
  oldPath?: string;
  additions: number;
  deletions: number;
  content?: string;
  diff?: string;
}

export interface CommitInfo {
  message: string;
  description?: string;
  author?: {
    name: string;
    email: string;
  };
  amend?: boolean;
  signoff?: boolean;
}

export interface SyncStatus {
  ahead: number;
  behind: number;
  conflicts: string[];
  lastSync?: string;
  remoteStatus: 'connected' | 'disconnected' | 'error';
}

export interface BranchInfo {
  name: string;
  type: 'local' | 'remote';
  current: boolean;
  upstream?: string;
  lastCommit: GitCommit;
  ahead: number;
  behind: number;
}

export interface OperationProgress {
  id: string;
  type: 'clone' | 'push' | 'pull' | 'commit' | 'merge';
  status: 'pending' | 'running' | 'success' | 'error';
  progress: number;
  message: string;
  startTime: string;
  endTime?: string;
  error?: string;
}

export interface PullOptions {
  remote?: string;
  branch?: string;
  rebase?: boolean;
}

export interface PushOptions {
  remote?: string;
  branch?: string;
  force?: boolean;
  tags?: boolean;
}

export interface CommitOptions {
  message: string;
  author?: {
    name: string;
    email: string;
  };
  amend?: boolean;
  signoff?: boolean;
}

export interface BranchOptions {
  name: string;
  startPoint?: string;
  checkout?: boolean;
  force?: boolean;
}

export interface MergeOptions {
  branch: string;
  strategy?: 'ours' | 'theirs' | 'recursive' | 'octopus' | 'resolve' | 'subtree';
  noFastForward?: boolean;
  squash?: boolean;
}

export interface RebaseOptions {
  onto?: string;
  upstream?: string;
  branch?: string;
  interactive?: boolean;
  preserve?: boolean;
}

export interface StashOptions {
  message?: string;
  includeUntracked?: boolean;
  keepIndex?: boolean;
}

// Git 状态相关类型
export interface GitStatus {
  current: string;
  tracking?: string;
  ahead: number;
  behind: number;
  files: GitStatusFile[];
  clean: boolean;
}

export interface GitStatusFile {
  path: string;
  index: GitFileStatus;
  working_tree: GitFileStatus;
  from?: string; // for renamed files
}

export type GitFileStatus =
  | ' '  // unmodified
  | 'M'  // modified
  | 'A'  // added
  | 'D'  // deleted
  | 'R'  // renamed
  | 'C'  // copied
  | 'U'  // updated but unmerged
  | '?'  // untracked
  | '!'; // ignored

// Git 配置类型
export interface GitConfig {
  user?: {
    name?: string;
    email?: string;
  };
  core?: {
    editor?: string;
    autocrlf?: boolean | 'input';
    safecrlf?: boolean;
    filemode?: boolean;
  };
  remote?: {
    [name: string]: {
      url: string;
      fetch: string;
    };
  };
  branch?: {
    [name: string]: {
      remote?: string;
      merge?: string;
    };
  };
}

// Git 日志相关类型
export interface GitLogOptions {
  since?: string;
  until?: string;
  author?: string;
  grep?: string;
  path?: string;
  maxCount?: number;
  skip?: number;
  reverse?: boolean;
  merges?: boolean;
  noMerges?: boolean;
}

export interface GitDiff {
  from: string;
  to: string;
  files: GitDiffFile[];
  stats: {
    additions: number;
    deletions: number;
    total: number;
  };
}

export interface GitDiffFile {
  path: string;
  oldPath?: string;
  status: GitFileStatus;
  additions: number;
  deletions: number;
  changes: number;
  binary: boolean;
  hunks: GitDiffHunk[];
}

export interface GitDiffHunk {
  oldStart: number;
  oldLines: number;
  newStart: number;
  newLines: number;
  header: string;
  lines: GitDiffLine[];
}

export interface GitDiffLine {
  type: 'context' | 'addition' | 'deletion';
  content: string;
  oldLineNumber?: number;
  newLineNumber?: number;
}

export interface ConflictFile {
  path: string;
  resolved: boolean;
  conflicts: Array<{
    id: string;
    startLine: number;
    endLine: number;
    currentContent: string;
    incomingContent: string;
  }>;
}
