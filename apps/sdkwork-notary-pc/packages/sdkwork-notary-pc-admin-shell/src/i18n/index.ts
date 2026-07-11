import enUS from './en-US/notary/shell/navigation.json';
import zhCN from './zh-CN/notary/shell/navigation.json';

export type NotaryPcAdminShellMessages = typeof zhCN;

function resolveRuntimeLanguage(): string {
  if (typeof document !== 'undefined' && document.documentElement.lang) {
    return document.documentElement.lang;
  }
  if (typeof navigator !== 'undefined') {
    return navigator.language;
  }
  return 'zh-CN';
}

export function getNotaryPcAdminShellMessages(
  language = resolveRuntimeLanguage(),
): NotaryPcAdminShellMessages {
  return language.toLowerCase().startsWith('zh') ? zhCN : enUS;
}
