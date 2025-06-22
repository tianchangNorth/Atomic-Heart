/**
 * 本地仓库相关类型定义
 */

export type RepositoryStatus = 'valid' | 'invalid' | 'unknown';

/**
 * 本地仓库接口
 */
export interface LocalRepository {
  /** 唯一标识符 */
  id: string;
  /** 仓库名称 */
  name: string;
  /** 本地路径 */
  path: string;
  /** 远程 URL（可选） */
  remoteUrl?: string;
  /** 当前分支（可选） */
  currentBranch?: string;
  /** 添加时间 */
  addedAt: string;
  /** 最后检查时间（可选） */
  lastChecked?: string;
  /** 仓库状态 */
  status: RepositoryStatus;
}

/**
 * 本地仓库存储数据结构
 */
export interface LocalRepositoryStore {
  /** 仓库列表 */
  repositories: LocalRepository[];
  /** 最后更新时间 */
  lastUpdated: string;
  /** 版本号 */
  version: string;
}

/**
 * 添加仓库的参数
 */
export interface AddRepositoryParams {
  /** 仓库名称 */
  name: string;
  /** 本地路径 */
  path: string;
  /** 远程 URL（可选） */
  remoteUrl?: string;
  /** 当前分支（可选） */
  currentBranch?: string;
}

/**
 * 更新仓库的参数
 */
export interface UpdateRepositoryParams {
  /** 仓库 ID */
  id: string;
  /** 更新的字段 */
  updates: Partial<Omit<LocalRepository, 'id' | 'addedAt'>>;
}

/**
 * 仓库验证结果
 */
export interface RepositoryValidation {
  /** 是否有效 */
  isValid: boolean;
  /** 验证消息 */
  message: string;
  /** 仓库信息（如果有效） */
  info?: {
    name: string;
    remoteUrl?: string;
    currentBranch?: string;
  };
}

/**
 * 仓库操作结果
 */
export interface RepositoryOperationResult {
  /** 操作是否成功 */
  success: boolean;
  /** 结果消息 */
  message: string;
  /** 返回的数据（可选） */
  data?: any;
}
