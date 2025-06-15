// Prism.js 安全加载器
import Prism from 'prismjs';

// 基础语言映射
const languageMap: Record<string, string> = {
  'js': 'javascript',
  'ts': 'typescript',
  'jsx': 'javascript',
  'tsx': 'typescript',
  'py': 'python',
  'rb': 'ruby',
  'sh': 'bash',
  'shell': 'bash',
  'yml': 'yaml',
  'vue': 'markup',
  'html': 'markup',
  'xml': 'markup',
  'svg': 'markup'
};

// 已加载的语言集合
const loadedLanguages = new Set<string>(['markup', 'css', 'clike', 'javascript']);

// 动态加载语言组件
export const loadLanguage = async (language: string): Promise<boolean> => {
  const normalizedLang = languageMap[language] || language;
  
  if (loadedLanguages.has(normalizedLang)) {
    return true;
  }

  try {
    // 根据语言加载对应的组件
    switch (normalizedLang) {
      case 'typescript':
        if (!loadedLanguages.has('javascript')) {
          await import('prismjs/components/prism-javascript');
          loadedLanguages.add('javascript');
        }
        await import('prismjs/components/prism-typescript');
        break;
      
      case 'python':
        await import('prismjs/components/prism-python');
        break;
      
      case 'java':
        if (!loadedLanguages.has('clike')) {
          await import('prismjs/components/prism-clike');
          loadedLanguages.add('clike');
        }
        await import('prismjs/components/prism-java');
        break;
      
      case 'cpp':
      case 'c++':
        if (!loadedLanguages.has('c')) {
          await import('prismjs/components/prism-c');
          loadedLanguages.add('c');
        }
        await import('prismjs/components/prism-cpp');
        break;
      
      case 'csharp':
      case 'c#':
        if (!loadedLanguages.has('clike')) {
          await import('prismjs/components/prism-clike');
          loadedLanguages.add('clike');
        }
        await import('prismjs/components/prism-csharp');
        break;
      
      case 'php':
        await import('prismjs/components/prism-php');
        break;
      
      case 'ruby':
        await import('prismjs/components/prism-ruby');
        break;
      
      case 'go':
        if (!loadedLanguages.has('clike')) {
          await import('prismjs/components/prism-clike');
          loadedLanguages.add('clike');
        }
        await import('prismjs/components/prism-go');
        break;
      
      case 'rust':
        if (!loadedLanguages.has('clike')) {
          await import('prismjs/components/prism-clike');
          loadedLanguages.add('clike');
        }
        await import('prismjs/components/prism-rust');
        break;
      
      case 'swift':
        if (!loadedLanguages.has('clike')) {
          await import('prismjs/components/prism-clike');
          loadedLanguages.add('clike');
        }
        await import('prismjs/components/prism-swift');
        break;
      
      case 'kotlin':
        if (!loadedLanguages.has('clike')) {
          await import('prismjs/components/prism-clike');
          loadedLanguages.add('clike');
        }
        await import('prismjs/components/prism-kotlin');
        break;
      
      case 'dart':
        if (!loadedLanguages.has('clike')) {
          await import('prismjs/components/prism-clike');
          loadedLanguages.add('clike');
        }
        await import('prismjs/components/prism-dart');
        break;
      
      case 'json':
        await import('prismjs/components/prism-json');
        break;
      
      case 'yaml':
        await import('prismjs/components/prism-yaml');
        break;
      
      case 'bash':
        await import('prismjs/components/prism-bash');
        break;
      
      case 'sql':
        await import('prismjs/components/prism-sql');
        break;
      
      case 'scss':
        if (!loadedLanguages.has('css')) {
          // CSS 是内置的，不需要导入
          loadedLanguages.add('css');
        }
        await import('prismjs/components/prism-scss');
        break;
      
      case 'markdown':
        await import('prismjs/components/prism-markdown');
        break;
      
      default:
        // 对于不支持的语言，返回 false
        return false;
    }
    
    loadedLanguages.add(normalizedLang);
    return true;
  } catch (err) {
    console.warn(`加载语言 ${normalizedLang} 失败:`, err);
    return false;
  }
};

// 安全的代码高亮函数
export const highlightCode = async (code: string, language: string): Promise<string> => {
  const normalizedLang = languageMap[language] || language;
  
  try {
    // 尝试加载语言
    await loadLanguage(normalizedLang);
    
    // 检查语言是否可用
    if (Prism.languages[normalizedLang]) {
      return Prism.highlight(code, Prism.languages[normalizedLang], normalizedLang);
    }
  } catch (err) {
    console.warn(`代码高亮失败 (${language}):`, err);
  }
  
  // 如果高亮失败，返回原始代码
  return code;
};

// 安全的全局高亮函数
export const highlightAll = (): void => {
  try {
    if (Prism && Prism.highlightAll) {
      Prism.highlightAll();
    }
  } catch (err) {
    console.warn('全局代码高亮失败:', err);
  }
};

// 获取支持的语言列表
export const getSupportedLanguages = (): string[] => {
  return Array.from(loadedLanguages);
};

export default Prism;
