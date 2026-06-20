import { NotaryHomePage } from '@sdkwork/notary-h5-notary';

export interface NotaryAppProps {
  route: string;
}

export function NotaryApp({ route }: NotaryAppProps) {
  if (route.startsWith('/notary')) {
    return <NotaryHomePage />;
  }

  return <NotaryHomePage />;
}
