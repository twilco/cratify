import * as React from 'react'
import { ChangeEvent, MouseEvent } from 'react'
import {
Button,
Input,
} from 'reactstrap'
import styled from 'styled-components'
import { signup } from '../api-sdk/sdk'

const ContentContainer = styled.div`
  text-align: center;
`

const PageHeader = styled.h1`
  margin-bottom: 50px;
`

const StyledInput = styled(Input)`
  margin-bottom: 25px;
`

interface IState {
  confirmPassword: string,
  password: string,
  passwordsMatch: boolean,
  username: string,
}

export default class Signup extends React.Component<{}, IState> {
  constructor(props: {}) {
    super(props)

    this.state = {
      confirmPassword: '',
      password: '',
      passwordsMatch: true,
      username: '',
    }
  }
  public render() {
    return (
      <div className="container">
        <div className="row justify-content-md-center">
          <ContentContainer className="col-4">
            <PageHeader>Sign up with Cratify</PageHeader>
            <StyledInput
              autoFocus={ true }
              value={ this.state.username }
              onChange={ this.usernameChanged }
              placeholder="username"
            />
            <StyledInput
              valid={ this.state.passwordsMatch }
              invalid={ !this.state.passwordsMatch }
              onChange={ this.passwordChanged }
              placeholder="password"
              type="password"
              value={ this.state.password }
            />
            <StyledInput
              valid={ this.state.passwordsMatch }
              invalid={ !this.state.passwordsMatch }
              onChange={ this.confirmPasswordChanged }
              placeholder="confirm password"
              type="password"
              value={ this.state.confirmPassword }
            />
            <Button
              color="primary"
              disabled={ !this.formIsValid() }
              onClick={ this.signupClicked }
            >
              Sign up
            </Button>
          </ContentContainer>
        </div>
      </div>
    )
  }
  private formIsValid = () => {
    return (this.state.password && this.state.username && this.state.passwordsMatch) || false
  }

  private confirmPasswordChanged = (evt: ChangeEvent<HTMLInputElement>) => {
    this.setState({
      confirmPassword: evt.target.value,
    })
    if (this.state.password === evt.target.value) {
      this.setState({
        passwordsMatch: true,
      })
    } else {
      this.setState({
        passwordsMatch: false,
      })
    }
  }

  private passwordChanged = (evt: ChangeEvent<HTMLInputElement>) => {
    this.setState({
      password: evt.target.value,
    })
    if (this.state.confirmPassword === evt.target.value) {
      this.setState({
        passwordsMatch: true,
      })
    } else {
      this.setState({
        passwordsMatch: false,
      })
    }
  }

  private usernameChanged = (evt: ChangeEvent<HTMLInputElement>) => {
    this.setState({
      username: evt.target.value,
    })
  }

  private signupClicked = async (evt: MouseEvent) => {
    if (this.formIsValid()) {
      const resp = await signup(this.state.username, this.state.password)
      console.log(resp)
    }
  }
}
