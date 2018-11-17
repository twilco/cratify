import i18n from 'i18next'
import langDetector from 'i18next-browser-languagedetector'
import translationsEn from './translations/en.json'

const resources = {
  en: {
    translation: translationsEn,
  }
}

i18n
  .use(langDetector)
  .init({
    fallbackLng: 'en',
    interpolation: {
      escapeValue: false // react is already safe from xss
    },
    keySeparator: false,
    react: {
      wait: true
    },
    resources
})
export default i18n