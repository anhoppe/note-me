import { AppState, Auth0Provider } from '@auth0/auth0-react';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import { createRoot } from 'react-dom/client';
import { StrictMode } from 'react';

import './index.css'
import App from './App.tsx'
import LoginButton from './login';

const onRedirectCallback = (appState?: AppState | undefined) => {
  console.log("Redirect callback triggered:", appState);
}

const providerConfig = {
  domain: "dev-40fxc54pknc85fku.us.auth0.com",
  clientId: "xmcJLDXZCnFO6pS7s9mbJnm0rNbNLVqx",
  onRedirectCallback,
  authorizationParams: {
    redirect_uri: window.location.origin + "/app"
  },
};

console.log(providerConfig.authorizationParams.redirect_uri);

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <Router>
        <Auth0Provider
          {...providerConfig}
        >
        <Routes>
          <Route path="/" element={<LoginButton />}/>
          <Route path="/app" element={<App />} />
        </Routes>
      </Auth0Provider>
    </Router>
  </StrictMode>
)
