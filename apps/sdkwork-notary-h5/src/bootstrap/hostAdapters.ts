export interface NotaryH5HostAdapters {
  platform: 'browser';
}

let hostAdapters: NotaryH5HostAdapters | null = null;

export function registerHostAdapters(): NotaryH5HostAdapters {
  if (!hostAdapters) {
    hostAdapters = { platform: 'browser' };
  }
  return hostAdapters;
}

export function getHostAdapters(): NotaryH5HostAdapters {
  return hostAdapters ?? registerHostAdapters();
}
