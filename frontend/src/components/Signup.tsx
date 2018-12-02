import { debounce } from 'debounce'
import { TranslationFunction } from 'i18next'
import * as React from 'react'
import { ChangeEvent } from 'react'
import {
Button,
Input,
} from 'reactstrap'
import styled from 'styled-components'
import { signup, usernameAvailable } from '../api-sdk/sdk'

const ContentContainer = styled.div`
  text-align: center;
`

const PageHeader = styled.h1`
  margin-bottom: 50px;
`

const StyledInput = styled(Input)<{ hasValidationMessage: boolean }>`
  margin-bottom: ${props => props.hasValidationMessage ? '0px' : '25px'};
  margin-top: 6px;
  border-color: 1px solid #ced4da;
`

const ValidationMessage = styled.div`
  /* Bootstrap override of class .invalid-feedback */
  margin-top: 0px !important;
`

interface IProps {
  t: TranslationFunction
}

interface IState {
  confirmPassword: string,
  password: string,
  passwordsClean: boolean,
  passwordsMatch: boolean,
  usernameClean: boolean,
  usernameTaken: boolean,
  username: string,
}

export default class Signup extends React.Component<IProps, IState> {
  private debouncedAvailabiltyCheck = debounce((username: string) => {
    usernameAvailable(username).then((available) => {
      if (typeof available === 'boolean') {
        this.setState({
          usernameClean: username === '',
          usernameTaken: !available,
        })
      }
    })
  }, 200)
  constructor(props: IProps) {
    super(props)

    this.state = {
      confirmPassword: '',
      password: '',
      passwordsClean: true,
      passwordsMatch: true,
      username: '',
      usernameClean: true,
      usernameTaken: false,
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
              hasValidationMessage={ this.state.usernameTaken }
              invalid={ this.state.usernameClean ? undefined : this.state.usernameTaken }
              onChange={ this.usernameChanged }
              placeholder={ t('username-lower') }
              valid={ this.state.usernameClean ? undefined : !this.state.usernameTaken }
              value={ this.state.username }
            />
            { this.state.usernameTaken &&
              <ValidationMessage className="invalid-feedback">
                username taken - please try another
              </ValidationMessage>
            }
            <StyledInput
              hasValidationMessage={ false }
              invalid={ this.state.passwordsClean ? undefined : !this.state.passwordsMatch }
              onChange={ this.passwordChanged }
              placeholder={ t('password-lower') }
              type="password"
              valid={ this.state.passwordsClean ? undefined : this.state.passwordsMatch }
              value={ this.state.password }
            />
            <StyledInput
              hasValidationMessage={ false }
              invalid={ this.state.passwordsClean ? undefined : !this.state.passwordsMatch }
              onChange={ this.confirmPasswordChanged }
              placeholder={ t('confirm-password-lower') }
              type="password"
              valid={ this.state.passwordsClean ? undefined : this.state.passwordsMatch }
              value={ this.state.confirmPassword }
            />
            <Button
              color="primary"
              disabled={ !this.formIsValid() }
              onClick={ this.signupClicked }
              type="button"
            >
              { t('sign-up') }
            </Button>
          </ContentContainer>
        </div>
      </div>
    )
  }
  private formIsValid = () => {
    return this.state.password && this.state.username && this.state.passwordsMatch && !this.state.usernameTaken
  }

  private confirmPasswordChanged = (evt: ChangeEvent<HTMLInputElement>) => {
    let newClean = this.state.passwordsClean
    let newMatch = this.state.passwordsMatch
    if (this.state.password === evt.target.value) {
      newClean = evt.target.value === ''
      newMatch = true
    } else {
      newClean = false
      newMatch = false
    }
    this.setState({
      confirmPassword: evt.target.value,
      passwordsClean: newClean,
      passwordsMatch: newMatch,
    })
  }

  private passwordChanged = (evt: ChangeEvent<HTMLInputElement>) => {
    let newClean = this.state.passwordsClean
    let newMatch = this.state.passwordsMatch
    if (this.state.confirmPassword === evt.target.value) {
      newClean = evt.target.value === ''
      newMatch = true
    } else {
      newClean = false
      newMatch = false
    }
    this.setState({
      password: evt.target.value,
      passwordsClean: newClean,
      passwordsMatch: newMatch,
    })
  }

  private usernameChanged = async (evt: ChangeEvent<HTMLInputElement>) => {
    this.setState({
      username: evt.target.value,
    })
    this.debouncedAvailabiltyCheck(evt.target.value)
  }

  private signupClicked = async () => {
    if (this.formIsValid()) {
      if (await usernameAvailable(this.state.username)) {
        const resp = await signup(this.state.username, this.state.password)
        console.log(resp)
      } else {
        this.setState({
          usernameTaken: true,
        })
      }
    }
  }
}
