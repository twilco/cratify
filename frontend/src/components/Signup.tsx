import { TranslationFunction } from 'i18next'
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
  border-color: 1px solid #ced4da;
`

interface IProps {
  t: TranslationFunction
}

interface IState {
  confirmPassword: string,
  password: string,
  passwordsDirty: boolean,
  passwordsMatch: boolean,
  username: string,
}

export default class Signup extends React.Component<IProps, IState> {
  constructor(props: IProps) {
    super(props)

    this.state = {
      confirmPassword: '',
      password: '',
      passwordsDirty: false,
      passwordsMatch: true,
      username: '',
    }
  }
  public render() {
    const { t } = this.props
    return (
      <div className="container">
        <div className="row justify-content-md-center">
          <ContentContainer className="col-4">
            <PageHeader>{ t('sign-up-with-cratify') }</PageHeader>
            <StyledInput
              autoFocus={ true }
              onChange={ this.usernameChanged }
              placeholder={ t('username-lower') }
              value={ this.state.username }
            />
            <StyledInput
              className={ this.state.passwordsDirty ? '' : 'cratify-clean-input' }
              invalid={ !this.state.passwordsMatch }
              onChange={ this.passwordChanged }
              placeholder={ t('password-lower') }
              type="password"
              valid={ this.state.passwordsMatch }
              value={ this.state.password }
            />
            <StyledInput
              className={ this.state.passwordsDirty ? '' : 'cratify-clean-input' }
              invalid={ !this.state.passwordsMatch }
              onChange={ this.confirmPasswordChanged }
              placeholder={ t('confirm-password-lower') }
              type="password"
              valid={ this.state.passwordsMatch }
              value={ this.state.confirmPassword }
            />
            <Button
              color="primary"
              disabled={ !this.formIsValid() }
              onClick={ this.signupClicked }
            >
              { t('sign-up') }
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
    let newDirty = this.state.passwordsDirty
    let newMatch = this.state.passwordsMatch
    if (this.state.password === evt.target.value) {
      newDirty = evt.target.value !== ''
      newMatch = true
    } else {
      newDirty = true
      newMatch = false
    }
    this.setState({
      confirmPassword: evt.target.value,
      passwordsDirty: newDirty,
      passwordsMatch: newMatch,
    })
  }

  private passwordChanged = (evt: ChangeEvent<HTMLInputElement>) => {
    let newDirty = this.state.passwordsDirty
    let newMatch = this.state.passwordsMatch
    if (this.state.confirmPassword === evt.target.value) {
      newDirty = evt.target.value !== ''
      newMatch = true
    } else {
      newDirty = true
      newMatch = false
    }
    this.setState({
      password: evt.target.value,
      passwordsDirty: newDirty,
      passwordsMatch: newMatch,
    })
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
