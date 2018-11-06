import * as React from 'react'
import { BrowserRouter, Route } from 'react-router-dom'
import styled from 'styled-components'

import About from './About'
import AppNav from './AppNav'
import Footer from './Footer'

const StyledFooter = styled(Footer)`
  margin-bottom: 35px;
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
        </ContentContainer>
        <StyledFooter/>
      </div>
    </BrowserRouter>
)

export default App
