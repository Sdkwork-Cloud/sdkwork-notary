import type { ComponentType } from 'react';

import type {
  NotaryCallOverlayProps,
  NotaryMediaPreviewProps,
  NotaryPcHostAdapter,
} from './notaryPcHost';

function joinClassNames(...inputs: Array<string | false | null | undefined>): string {
  return inputs.filter(Boolean).join(' ');
}

function BrowserCallOverlay({ isOpen, onClose, callerName, name }: NotaryCallOverlayProps) {
  if (!isOpen) {
    return null;
  }

  const displayName = callerName ?? name ?? 'Unknown';

  return (
    <div
      role="dialog"
      aria-label="Call overlay"
      style={{
        position: 'fixed',
        inset: 0,
        background: 'rgba(0,0,0,0.45)',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        zIndex: 1000,
      }}
    >
      <div style={{ background: '#fff', padding: 24, borderRadius: 8, minWidth: 320 }}>
        <p>Calling {displayName}</p>
        <button type="button" onClick={onClose}>
          Close
        </button>
      </div>
    </div>
  );
}

function BrowserMediaViewer({ isOpen, onClose, type, url, name }: NotaryMediaPreviewProps) {
  if (!isOpen) {
    return null;
  }

  return (
    <div
      role="dialog"
      aria-label="Media preview"
      style={{
        position: 'fixed',
        inset: 0,
        background: 'rgba(0,0,0,0.6)',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        zIndex: 1000,
      }}
      onClick={onClose}
    >
      <div
        style={{ background: '#111', padding: 16, borderRadius: 8, maxWidth: '90vw', maxHeight: '90vh' }}
        onClick={(event) => event.stopPropagation()}
      >
        <p style={{ color: '#fff', marginBottom: 8 }}>{name}</p>
        {type === 'image' ? (
          <img src={url} alt={name} style={{ maxWidth: '80vw', maxHeight: '70vh' }} />
        ) : (
          <a href={url} target="_blank" rel="noopener noreferrer" style={{ color: '#8cf' }}>
            Open media
          </a>
        )}
        <div style={{ marginTop: 12 }}>
          <button type="button" onClick={onClose}>
            Close
          </button>
        </div>
      </div>
    </div>
  );
}

function sanitizeLinkHref(url: string): string {
  const trimmed = url.trim();
  if (!trimmed) {
    return '';
  }
  try {
    const parsed = new URL(trimmed, window.location.origin);
    if (parsed.protocol === 'http:' || parsed.protocol === 'https:') {
      return parsed.toString();
    }
  } catch {
    return '';
  }
  return '';
}

export function createDefaultBrowserHostAdapter(): NotaryPcHostAdapter {
  const CallOverlay = BrowserCallOverlay as ComponentType<NotaryCallOverlayProps>;
  const MediaViewer = BrowserMediaViewer as ComponentType<NotaryMediaPreviewProps>;

  return {
    toast(message, variant = 'info') {
      const prefix = variant === 'error' ? '[error]' : variant === 'success' ? '[ok]' : '[info]';
      console.log(`${prefix} ${message}`);
    },
    openExternalUrl(url) {
      const safeUrl = sanitizeLinkHref(url);
      if (safeUrl) {
        window.open(safeUrl, '_blank', 'noopener,noreferrer');
      }
    },
    createDefaultAvatar(seed: string) {
      const label = encodeURIComponent(seed.slice(0, 2).toUpperCase() || 'NA');
      return `https://ui-avatars.com/api/?name=${label}&background=random`;
    },
    CallOverlay,
    MediaViewer,
    sanitizeLinkHref,
    cn: joinClassNames,
    resolveInitialLanguage() {
      return navigator.language === 'en-US' ? 'en-US' : 'zh-CN';
    },
  };
}
