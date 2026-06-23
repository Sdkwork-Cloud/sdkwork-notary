import i18next from 'i18next';
import { initReactI18next } from 'react-i18next';
import { getNotaryPcHost, tryGetNotaryPcHost } from '@sdkwork/notary-pc-commons';
import zhCN from './locales/zh-CN.json';
import enUS from './locales/en-US.json';

const SUPPORTED_LANGUAGES = ['zh-CN', 'en-US'] as const;
type SupportedLanguage = typeof SUPPORTED_LANGUAGES[number];

function normalizeLanguage(value: unknown): SupportedLanguage {
  return SUPPORTED_LANGUAGES.includes(value as SupportedLanguage)
    ? (value as SupportedLanguage)
    : 'zh-CN';
}

function resolveInitialLanguage(): SupportedLanguage {
  const host = tryGetNotaryPcHost();
  const resolved = host?.resolveInitialLanguage?.();
  return normalizeLanguage(resolved ?? 'zh-CN');
}

const i18n = i18next.createInstance();

i18n.use(initReactI18next).init({
  resources: {
    'zh-CN': { notary: zhCN },
    'en-US': { notary: enUS },
  },
  lng: resolveInitialLanguage(),
  fallbackLng: 'zh-CN',
  ns: ['notary'],
  defaultNS: 'notary',
  interpolation: { escapeValue: false },
});

if (typeof window !== 'undefined') {
  try {
    const unsubscribe = getNotaryPcHost().onLanguageChange?.((lang) => {
      const nextLanguage = normalizeLanguage(lang);
      if (i18n.language !== nextLanguage) {
        void i18n.changeLanguage(nextLanguage);
      }
    });
    if (unsubscribe) {
      window.addEventListener('beforeunload', unsubscribe);
    }
  } catch {
    // Host adapter may not be configured yet during module initialization.
  }
}

export default i18n;
