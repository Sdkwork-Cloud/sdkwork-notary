import { createIamRuntime, finalizeIamRuntime } from './iamRuntime';
import { registerHostAdapters } from './hostAdapters';
import { createRoutes } from './routes';
import { bootstrapSdkClients } from './sdkClients';

export function bootstrap() {
  createIamRuntime();
  bootstrapSdkClients();
  finalizeIamRuntime();
  registerHostAdapters();
  createRoutes();
}
