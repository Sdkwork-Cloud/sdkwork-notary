import { BrowserRouter, Navigate, Route, Routes } from 'react-router-dom';
import { SdkworkSessionAuthBrowserRoot } from '@sdkwork/auth-pc-react';

import { NotaryPcRoutes, NOTARY_PC_HOME_PATH } from '@sdkwork/notary-pc-shell';

import { AuthGate } from './AuthGate';
import { bootstrap } from './bootstrap/runtime';

import './index.css';

bootstrap();

export default function App() {
  return (
    <BrowserRouter>
      <SdkworkSessionAuthBrowserRoot>
      <AuthGate>
        <Routes>
          <Route path="/" element={<Navigate replace to={NOTARY_PC_HOME_PATH} />} />
          <Route path="/notary/*" element={<NotaryPcRoutes />} />
          <Route path="*" element={<Navigate replace to={NOTARY_PC_HOME_PATH} />} />
        </Routes>
      </AuthGate>
          </SdkworkSessionAuthBrowserRoot>
    </BrowserRouter>
  );
}
