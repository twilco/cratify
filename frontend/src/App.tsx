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

const App = () => (
    <BrowserRouter>
      <div>
        <GlobalStyles/>
        <StyledAppNav/>
        <ContentContainer>
          <Route path="/" exact={true} component={About} />
          <Route path="/signup" exact={true} component={Signup} />
        </ContentContainer>
        <StyledFooter/>
      </div>
    </BrowserRouter>
)

export default App
