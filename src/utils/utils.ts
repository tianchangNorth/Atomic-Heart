/**
 * 从 Git URL 中提取仓库名称
 * 支持 SSH（git@host:user/repo.git）和 HTTPS（https://host/user/repo.git）格式
 * @param url 仓库 URL 字符串
 * @returns 仓库名，解析失败返回 'Unknown Repository'
 */
export const extractRepositoryName = (url: string): string => {
  if (!url || url.trim().length === 0) return 'Unknown Repository';

  try {
    let repoName = url.trim();

    // 移除 .git 后缀
    if (repoName.endsWith('.git')) {
      repoName = repoName.slice(0, -4);
    }

    // SSH 格式: git@host:user/repo
    if (repoName.includes('@') && repoName.includes(':')) {
      const match = repoName.match(/@[^:]+:(.+)/);
      if (match && match[1]) {
        repoName = match[1];
      }
    }
    // HTTPS 或 HTTP 格式
    else if (repoName.startsWith('http://') || repoName.startsWith('https://')) {
      try {
        const urlObj = new URL(repoName);
        repoName = urlObj.pathname; // 形如 /user/repo
      } catch {
        // URL 解析失败，尝试简易截取
        const protocolIndex = repoName.indexOf('://');
        if (protocolIndex !== -1) {
          repoName = repoName.substring(protocolIndex + 3);
          const slashIndex = repoName.indexOf('/');
          if (slashIndex !== -1) {
            repoName = repoName.substring(slashIndex);
          }
        }
      }
    }

    // 去除开头的斜杠
    repoName = repoName.replace(/^\/+/, '');

    // 提取最后一段作为仓库名
    const parts = repoName.split('/');
    repoName = parts.length > 0 ? parts[parts.length - 1] : repoName;

    // 过滤掉不允许的字符，只保留字母数字、下划线、破折号、点
    repoName = repoName.replace(/[^\w.-]/g, '');

    return repoName || 'Unknown Repository';
  } catch {
    return 'Unknown Repository';
  }
};
