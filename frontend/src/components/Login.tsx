import { TranslationFunction } from 'i18next'
import * as React from 'react'
import { ChangeEvent } from 'react'
import {
  Button,
  Input,
} from 'reactstrap'
import styled from 'styled-components'
import { login } from '../api-sdk/sdk'

const ContentContainer = styled.div`
  text-align: center;
`

const PageHeader = styled.h1`
  margin-bottom: 50px;
`

const StyledInput = styled(Input)`
  margin-bottom: 25px;
  margin-top: 6px;
  border-color: 1px solid #ced4da;
`

interface IProps {
  t: TranslationFunction
}

interface IState {
  password: string,
  username: string,
}

export default class Login extends React.Component<IProps, IState> {
  constructor(props: IProps) {
    super(props)

    this.state = {
      password: '',
      username: '',
    }
  }
  public render() {
    const { t } = this.props
    return (
      <div className="container">
        <div className="row justify-content-md-center">
          <ContentContainer className="col-4">
            <PageHeader>{ t('login-to-cratify') }</PageHeader>
            <StyledInput
              autoFocus={ true }
              onChange={ this.usernameChanged }
              placeholder={ t('username-lower') }
              value={ this.state.username }
            />
            <StyledInput
              onChange={ this.passwordChanged }
              placeholder={ t('password-lower') }
              type="password"
              value={ this.state.password }
            />
            <Button
              color="primary"
              disabled={ !this.formIsValid() }
              onClick={ this.loginClicked }
              type="button"
            >
              { t('log-in') }
            </Button>
          </ContentContainer>
        </div>
      </div>
    )
  }
  private formIsValid = () => {
    return this.state.password && this.state.username
  }

  private passwordChanged = (evt: ChangeEvent<HTMLInputElement>) => {
    this.setState({
      password: evt.target.value,
    })
  }

  private usernameChanged = async (evt: ChangeEvent<HTMLInputElement>) => {
    this.setState({
      username: evt.target.value,
    })
  }

  private loginClicked = async () => {
    if (this.formIsValid()) {
      console.log(await login(this.state.username, this.state.password))
    }
  }
}
