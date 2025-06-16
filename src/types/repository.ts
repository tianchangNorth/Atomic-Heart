// 仓库相关类型定义

export interface Repository {
  forksCount: number;
  id: string;
  name: string;
  full_name: string;
  description?: string;
  private: boolean;
  owner: {
    id: string;
    login: string;
    avatar_url: string;
    type: 'User' | 'Organization';
  };
  html_url: string;
  clone_url: string;
  git_url: string;
  ssh_url: string;
  default_branch: string;
  language?: string;
  languages_url?: string;
  size: number;
  stars_count: number;
  watchers_count: number;
  forks_count: number;
  open_issues_count: number;
  createdAt: string;
  updatedAt: string;
  pushed_at?: string;
  archived: boolean;
  disabled: boolean;
  visibility: 'public' | 'private' | 'internal';
  topics?: string[];
  license?: {
    key: string;
    name: string;
    spdx_id: string;
    url?: string;
  };
}

export interface RepositoryStats {
  commits_count: number;
  branches_count: number;
  tags_count: number;
  contributors_count: number;
  files_count: number;
  total_size: number;
}

export interface Branch {
  name: string;
  commit: {
    sha: string;
    url?: string | null;
    html_url?: string | null;
    author?: {
      name: string;
      email: string;
      date: string;
    } | null;
    committer?: {
      name: string;
      email: string;
      date: string;
    } | null;
    commit?: {
      message: string;
      author?: {
        name: string;
        email: string;
        date: string;
      } | null;
      committer?: {
        name: string;
        email: string;
        date: string;
      } | null;
    } | null;
    comments_url?: string | null;
    parents?: Array<{
      sha: string;
      url?: string | null;
      html_url?: string | null;
    }>;
    files?: any[];
    stats?: {
      additions: number;
      deletions: number;
      total: number;
    } | null;
  };
  protected: boolean;
}

export interface FileTreeItem {
  name: string;
  path: string;
  type: 'file' | 'dir' | 'symlink';
  size?: number;
  sha: string;
  url: string;
  html_url: string;
  download_url?: string;
  last_commit?: {
    sha: string;
    message: string;
    author: {
      name: string;
      date: string;
    };
  };
  children?: FileTreeItem[];
  expanded?: boolean;
  loading?: boolean;
}

export interface FileContent {
  name: string;
  path: string;
  sha: string;
  size: number;
  url: string;
  html_url: string;
  git_url: string;
  download_url: string;
  type: 'file';
  content: string;
  encoding: 'base64' | 'utf-8';
}

export interface ReadmeFile {
  name: string;
  path: string;
  content: string;
  encoding: 'base64' | 'utf-8';
  size: number;
  html_url: string;
}

export interface RepositoryLanguages {
  [language: string]: number;
}

export interface Contributor {
  id: string;
  login: string;
  avatar_url: string;
  html_url: string;
  contributions: number;
  type: 'User' | 'Bot';
}

export interface Tag {
  name: string;
  zipball_url: string;
  tarball_url: string;
  commit: {
    sha: string;
    url: string;
  };
}

export interface Release {
  id: string;
  tag_name: string;
  name: string;
  body: string;
  draft: boolean;
  prerelease: boolean;
  created_at: string;
  published_at: string;
  author: {
    id: string;
    login: string;
    avatar_url: string;
  };
  assets: Array<{
    id: string;
    name: string;
    size: number;
    download_count: number;
    browser_download_url: string;
  }>;
}

// API 响应类型
export interface RepositoryDetailResponse {
  repository: Repository;
  stats: RepositoryStats;
  branches: Branch[];
  readme?: ReadmeFile;
  languages: RepositoryLanguages;
  contributors: Contributor[];
  tags: Tag[];
  latest_release?: Release;
}

// 组件 Props 类型
export interface RepoInfoProps {
  repository: Repository;
  stats?: RepositoryStats | null;
  languages: RepositoryLanguages;
  contributors: Contributor[];
}

export interface BranchSelectorProps {
  branches: Branch[];
  currentBranch: string;
  loading?: boolean;
}

export interface FileTreeProps {
  items: FileTreeItem[];
  currentPath?: string;
  loading?: boolean;
}

export interface ReadmeViewerProps {
  readme?: ReadmeFile | null;
  loading?: boolean;
}

// 事件类型
export interface BranchChangeEvent {
  branch: string;
  commit: string;
}

export interface FileSelectEvent {
  file: FileTreeItem;
  path: string;
}

export interface DirectoryToggleEvent {
  directory: FileTreeItem;
  expanded: boolean;
}
