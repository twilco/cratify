import * as React from 'react'
import {
  Button,
  Input,
} from 'reactstrap'
import styled from 'styled-components'

const ContentContainer = styled.div`
  text-align: center;
`

const PageHeader = styled.h1`
  margin-bottom: 50px;
`

const StyledInput = styled(Input)`
  margin-bottom: 25px;
`

export default class Signup extends React.Component<{}, {}> {
  constructor(props: {}) {
    super(props)
  }

  public render() {
    return (
      <div className="container">
        <div className="row justify-content-md-center">
          <ContentContainer className="col-4">
            <PageHeader>Sign up with Cratify</PageHeader>
            <StyledInput placeholder="username" />
            <StyledInput placeholder="password" />
            <StyledInput placeholder="confirm password" />
            <Button color="primary">Sign up</Button>
          </ContentContainer>
        </div>
      </div>
    )
  }
}
