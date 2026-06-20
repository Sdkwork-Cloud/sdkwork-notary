import { BrowserRouter, Navigate, Route, Routes, useLocation } from 'react-router-dom';

import { AuthGate } from './AuthGate';
import { NotaryApp } from './NotaryApp';
import { NOTARY_APP_HOME_PATH } from './constants/appRoutes';
import { bootstrap } from './bootstrap/runtime';

import './index.css';

bootstrap();

function AppShell() {
  const location = useLocation();
  const route = location.pathname;

  if (route === '/' || route === '') {
    return <Navigate replace to={NOTARY_APP_HOME_PATH} />;
  }

  return (
    <AuthGate>
      <Routes>
        <Route path="/notary/*" element={<NotaryApp route={route} />} />
        <Route path="*" element={<Navigate replace to={NOTARY_APP_HOME_PATH} />} />
      </Routes>
    </AuthGate>
  );
}

export default function App() {
  return (
    <BrowserRouter>
      <AppShell />
    </BrowserRouter>
  );
}
