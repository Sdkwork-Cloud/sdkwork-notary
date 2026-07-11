const UUID_BYTE_COUNT = 16;

export function createNotaryCaseIntentIdempotencyKey(): string {
  const cryptoApi = typeof globalThis.crypto === 'undefined' ? undefined : globalThis.crypto;
  if (cryptoApi && typeof cryptoApi.randomUUID === 'function') {
    return cryptoApi.randomUUID();
  }
  if (!cryptoApi || typeof cryptoApi.getRandomValues !== 'function') {
    throw new Error('Secure Web Crypto random generation is unavailable');
  }

  const bytes = cryptoApi.getRandomValues(new Uint8Array(UUID_BYTE_COUNT));
  bytes[6] = (bytes[6] & 0x0f) | 0x40;
  bytes[8] = (bytes[8] & 0x3f) | 0x80;
  const hex = Array.from(bytes, (byte) => byte.toString(16).padStart(2, '0')).join('');
  return [
    hex.slice(0, 8),
    hex.slice(8, 12),
    hex.slice(12, 16),
    hex.slice(16, 20),
    hex.slice(20),
  ].join('-');
}

export function resolveNotaryCaseIdempotencyKey(value?: string): string {
  const callerKey = value?.trim();
  return callerKey || createNotaryCaseIntentIdempotencyKey();
}
