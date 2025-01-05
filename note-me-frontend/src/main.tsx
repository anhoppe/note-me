import { Auth0Provider } from '@auth0/auth0-react';
import { createRoot } from 'react-dom/client';
import { StrictMode } from 'react';

import './index.css'
import App from './App.tsx'
import LoginButton from './login';

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <Auth0Provider
      domain="dev-40fxc54pknc85fku.us.auth0.com"
      clientId="xmcJLDXZCnFO6pS7s9mbJnm0rNbNLVqx"
      authorizationParams={{
        redirect_uri: window.location.origin,
      }}
    >
      <LoginButton />
      <App />
    </Auth0Provider>
  </StrictMode>
)
