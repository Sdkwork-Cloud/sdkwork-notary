import i18next from 'i18next';
import { initReactI18next } from 'react-i18next';
import zhCN from './locales/zh-CN.json';
import enUS from './locales/en-US.json';
import {
  resolveNotaryHostLanguage,
  subscribeNotaryHostLanguage,
  syncNotaryHostLanguage,
} from './hostLanguageBridge';

const i18n = i18next.createInstance();

i18n.use(initReactI18next).init({
  resources: {
    'zh-CN': { notary: zhCN },
    'en-US': { notary: enUS },
  },
  lng: resolveNotaryHostLanguage(),
  fallbackLng: 'zh-CN',
  ns: ['notary'],
  defaultNS: 'notary',
  interpolation: { escapeValue: false },
});

export function syncNotaryI18nFromHost(): void {
  syncNotaryHostLanguage(i18n);
}

export function subscribeNotaryI18nFromHost(): (() => void) | undefined {
  return subscribeNotaryHostLanguage(i18n);
}

export default i18n;
