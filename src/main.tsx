import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import './styles.css';
import { I18nextProvider } from 'react-i18next';
import i18n from './i18n';

const rootNode = document.getElementById('root');
if (rootNode != null) {
  ReactDOM.createRoot(rootNode).render(
    <React.StrictMode>
      <I18nextProvider i18n={i18n}>
        <App />
      </I18nextProvider>
    </React.StrictMode>
  );
}
