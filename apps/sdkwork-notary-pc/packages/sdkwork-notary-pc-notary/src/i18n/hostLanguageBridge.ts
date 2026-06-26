import type { i18n as I18nInstance } from 'i18next';
import { getNotaryPcHost, tryGetNotaryPcHost } from '@sdkwork/notary-pc-commons';

const SUPPORTED_LANGUAGES = ['zh-CN', 'en-US'] as const;
export type NotaryLanguage = (typeof SUPPORTED_LANGUAGES)[number];

export function normalizeNotaryLanguage(value: unknown): NotaryLanguage {
  return SUPPORTED_LANGUAGES.includes(value as NotaryLanguage)
    ? (value as NotaryLanguage)
    : 'zh-CN';
}

export function resolveNotaryHostLanguage(): NotaryLanguage {
  const host = tryGetNotaryPcHost();
  const resolved = host?.resolveInitialLanguage?.();
  return normalizeNotaryLanguage(resolved ?? 'zh-CN');
}

export function syncNotaryHostLanguage(i18n: I18nInstance): void {
  try {
    const hostLanguage = tryGetNotaryPcHost()?.resolveInitialLanguage?.();
    if (!hostLanguage) {
      return;
    }

    const nextLanguage = normalizeNotaryLanguage(hostLanguage);
    if (i18n.language !== nextLanguage) {
      void i18n.changeLanguage(nextLanguage);
    }
  } catch {
    // Host adapter may not be configured during standalone bootstrap.
  }
}

export function subscribeNotaryHostLanguage(
  i18n: I18nInstance,
): (() => void) | undefined {
  try {
    return getNotaryPcHost().onLanguageChange?.((language) => {
      const nextLanguage = normalizeNotaryLanguage(language);
      if (i18n.language !== nextLanguage) {
        void i18n.changeLanguage(nextLanguage);
      }
    });
  } catch {
    return undefined;
  }
}
