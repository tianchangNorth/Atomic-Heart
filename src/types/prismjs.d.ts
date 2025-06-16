declare module 'prismjs/components/prism-clike' {
  import { Prism } from 'prismjs';
  const clike: Prism.LanguageDefinition;
  export default clike;
}

declare module 'prismjs/components/*' {
  import { Prism } from 'prismjs';
  const language: Prism.LanguageDefinition;
  export default language;
}
