export interface NotaryAppRouteDefinition {
  path: string;
  label: string;
}

export function createNotaryAppRoutes(): NotaryAppRouteDefinition[] {
  return [
    { path: '/notary', label: 'Workbench' },
  ];
}
