import i18n from 'i18next'
import langDetector from 'i18next-browser-languagedetector'
import translationsEn from './translations/en.json'

/* Besides the official react-18next docs, I found these resources helpful:
    - https://github.com/arkross/arkross.github.io/wiki/Using-react-i18next-Trans-Component
 */

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