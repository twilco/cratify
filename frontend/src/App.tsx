import * as React from 'react'
import { BrowserRouter, Route } from 'react-router-dom'
import styled from 'styled-components'

import About from './About'
import AppNav from './AppNav'
import Footer from './Footer'
import Signup from './Signup'

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

const App = () => (
    <BrowserRouter>
      <div>
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
