import { createIamRuntime } from './iamRuntime';
import { createRoutes } from './routes';
import { bootstrapSdkClients } from './sdkClients';

export function bootstrap() {
  createIamRuntime();
  bootstrapSdkClients();
  createRoutes();
}
