import enUS from './en-US/notary/merchandise/matters.json';
import zhCN from './zh-CN/notary/merchandise/matters.json';

export type NotaryMatterMessages = typeof zhCN;

function resolveRuntimeLanguage(): string {
  if (typeof document !== 'undefined' && document.documentElement.lang) {
    return document.documentElement.lang;
  }
  if (typeof navigator !== 'undefined') {
    return navigator.language;
  }
  return 'zh-CN';
}

export function getNotaryMatterMessages(
  language = resolveRuntimeLanguage(),
): NotaryMatterMessages {
  return language.toLowerCase().startsWith('zh') ? zhCN : enUS;
}

export function interpolateNotaryMatterMessage(
  template: string,
  values: Readonly<Record<string, string | number>>,
): string {
  return Object.entries(values).reduce(
    (message, [key, value]) => message.replaceAll(`{${key}}`, String(value)),
    template,
  );
}
