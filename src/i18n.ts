import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import en from './i18n/translations/en.json';

i18n
  .use(initReactI18next)
  .init({
    lng: 'en',
    resources: {
      en: {
        translation: en
      }
    }
  })
  .catch((reason) => {
    console.error(reason);
  });

export default i18n;
