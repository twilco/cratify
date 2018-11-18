/* tslint:disable:jsx-no-lambda - necessary for rendering components inside a Route with props. */
import { TranslationFunction } from 'i18next'
import * as React from 'react'
import { BrowserRouter, Route } from 'react-router-dom'
import styled, { createGlobalStyle } from 'styled-components'

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

const GlobalStyles = createGlobalStyle`
  /* Override some Bootstrap styles for valid and clean inputs. */
  .cratify-clean-input {
    border-color: #ced4da !important;
  }
  .cratify-clean-input:focus {
    color: #495057 !important;
    background-color: #fff !important;
    border-color: #80bdff !important;
    outline: 0 !important;
    -webkit-box-shadow: 0 0 0 0.2rem rgba(0,123,255,.25) !important;
    box-shadow: 0 0 0 0.2rem rgba(0,123,255,.25) !important;
  }
`

interface IProps {
  t: TranslationFunction
}

const App = ({ t } : IProps) => (
    <BrowserRouter>
      <div>
        <GlobalStyles/>
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
