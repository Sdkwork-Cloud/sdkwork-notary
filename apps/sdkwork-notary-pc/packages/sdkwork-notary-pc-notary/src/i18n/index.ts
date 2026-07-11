import i18next from 'i18next';
import { initReactI18next } from 'react-i18next';
import enCaseDetail from './en-US/notary/notary/case-detail.json';
import enCreateCase from './en-US/notary/notary/create-case.json';
import enFeedback from './en-US/notary/notary/feedback.json';
import enParty from './en-US/notary/notary/party.json';
import enPrint from './en-US/notary/notary/print.json';
import enSignature from './en-US/notary/notary/signature.json';
import enTaskList from './en-US/notary/notary/task-list.json';
import enWorkspace from './en-US/notary/notary/workspace.json';
import {
  resolveNotaryHostLanguage,
  subscribeNotaryHostLanguage,
  syncNotaryHostLanguage,
} from './hostLanguageBridge';
import zhCaseDetail from './zh-CN/notary/notary/case-detail.json';
import zhCreateCase from './zh-CN/notary/notary/create-case.json';
import zhFeedback from './zh-CN/notary/notary/feedback.json';
import zhParty from './zh-CN/notary/notary/party.json';
import zhPrint from './zh-CN/notary/notary/print.json';
import zhSignature from './zh-CN/notary/notary/signature.json';
import zhTaskList from './zh-CN/notary/notary/task-list.json';
import zhWorkspace from './zh-CN/notary/notary/workspace.json';

const i18n = i18next.createInstance();
const zhCN = {
  ...zhWorkspace,
  ...zhTaskList,
  ...zhCaseDetail,
  ...zhPrint,
  ...zhParty,
  ...zhSignature,
  ...zhCreateCase,
  ...zhFeedback,
};
const enUS = {
  ...enWorkspace,
  ...enTaskList,
  ...enCaseDetail,
  ...enPrint,
  ...enParty,
  ...enSignature,
  ...enCreateCase,
  ...enFeedback,
};

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
