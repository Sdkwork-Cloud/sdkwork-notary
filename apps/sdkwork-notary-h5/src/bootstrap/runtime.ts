import { createIamRuntime } from './iamRuntime';
import { registerHostAdapters } from './hostAdapters';
import { createRoutes } from './routes';
import { bootstrapSdkClients } from './sdkClients';

export function bootstrap() {
  createIamRuntime();
  registerHostAdapters();
  bootstrapSdkClients();
  createRoutes();
}
