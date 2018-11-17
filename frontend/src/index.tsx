import 'bootstrap/dist/css/bootstrap.min.css'
import * as React from 'react'
import * as ReactDOM from 'react-dom'
import { I18nextProvider, NamespacesConsumer } from 'react-i18next'
import App from './App'
import i18n from './i18n'
import registerServiceWorker from './registerServiceWorker'

ReactDOM.render(
  <I18nextProvider i18n={ i18n } defaultNS="translation">
      { /* These namespaces should match the 'resources' JSON object set up in the i8n.js bootstrap code. */ }
      <NamespacesConsumer ns={['translation']}>
        {
          (t) => {
            return <App t={t} />
          }
        }
      </NamespacesConsumer>
  </I18nextProvider>,
  document.getElementById('root') as HTMLElement,
)
registerServiceWorker()
