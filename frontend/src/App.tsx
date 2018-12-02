/* tslint:disable:jsx-no-lambda - necessary for rendering components inside a Route with props. */
import { TranslationFunction } from 'i18next'
import * as React from 'react'
import { BrowserRouter, Route } from 'react-router-dom'
import styled from 'styled-components'

import About from './components/About'
import AppNav from './components/AppNav'
import Footer from './components/Footer'
import Signup from './components/Signup'

const StyledFooter = styled(Footer)`
  margin-top: 25px;
  margin-bottom: 25px;
`

const ContentContainer = styled.div`
  min-height: 100%;
`

const StyledAppNav = styled(AppNav)`
  margin-bottom: 25px;
`

interface IProps {
  t: TranslationFunction
}

const App = ({ t } : IProps) => (
    <BrowserRouter>
      <div>
        <StyledAppNav t={ t } />
        <ContentContainer>
          <Route path="/" exact={ true } render={ props => <About { ...props } t={ t } /> } />
          <Route path="/signup" exact={ true } render={ props => <Signup { ...props } t={ t } /> } />
        </ContentContainer>
        <StyledFooter t={ t }/>
      </div>
    </BrowserRouter>
)

export default App
