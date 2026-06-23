import type { ComponentType } from 'react';

export type NotaryToastVariant = 'info' | 'success' | 'error';

export interface NotaryCallOverlayProps {
  isOpen: boolean;
  onClose: () => void;
  conversationId?: string;
  type?: string;
  callerName?: string;
  name?: string;
  callerAvatar?: string;
  [key: string]: unknown;
}

export interface NotaryMediaPreviewProps {
  isOpen: boolean;
  onClose: () => void;
  type: 'image' | 'video' | 'pdf' | string;
  url: string;
  name: string;
  [key: string]: unknown;
}

export interface NotaryPcHostAdapter {
  toast(message: string, variant?: NotaryToastVariant): void;
  openExternalUrl(url: string): void;
  createDefaultAvatar(seed: string): string;
  CallOverlay: ComponentType<NotaryCallOverlayProps>;
  MediaViewer: ComponentType<NotaryMediaPreviewProps>;
  sanitizeLinkHref(url: string): string;
  cn(...inputs: Array<string | false | null | undefined>): string;
  onLanguageChange?(listener: (lang: string) => void): () => void;
  resolveInitialLanguage?(): string;
}

let hostAdapter: NotaryPcHostAdapter | null = null;

export function configureNotaryPcHost(adapter: NotaryPcHostAdapter): void {
  hostAdapter = adapter;
}

export function getNotaryPcHost(): NotaryPcHostAdapter {
  if (!hostAdapter) {
    throw new Error('Notary PC host adapter is not configured. Call configureNotaryPcHost first.');
  }
  return hostAdapter;
}

export function tryGetNotaryPcHost(): NotaryPcHostAdapter | null {
  return hostAdapter;
}

export function notaryToast(message: string, variant?: NotaryToastVariant): void {
  getNotaryPcHost().toast(message, variant);
}

export function notaryCn(...inputs: Array<string | false | null | undefined>): string {
  return getNotaryPcHost().cn(...inputs);
}

export function notarySanitizeLinkHref(url: string): string {
  return getNotaryPcHost().sanitizeLinkHref(url);
}
