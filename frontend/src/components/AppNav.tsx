import * as React from 'react'
import {
  Nav,
  Navbar,
  NavbarBrand,
  NavbarToggler,
  NavItem,
  NavLink,
} from 'reactstrap'
import styled from 'styled-components'

interface IProps {
  className?: string
}

interface IState {
  isOpen: boolean
}

const StyledNavbar = styled(Navbar)`
    position: relative;
    display: -ms-flexbox;
    display: flex;
    -ms-flex-wrap: wrap;
    flex-wrap: wrap;
    -ms-flex-align: center;
    align-items: center;
    -ms-flex-pack: justify;
    justify-content: space-around;
    padding: .5rem 1rem;
`

export default class AppNav extends React.Component<IProps, IState> {
  constructor(props: IProps) {
    super(props)

    this.toggle = this.toggle.bind(this)
    this.state = {
      isOpen: false,
    }
  }

  public render() {
    return (
      <StyledNavbar className={this.props.className} color="light" light={true} expand="md">
          <NavbarBrand href="/">Cratify</NavbarBrand>
          <NavbarToggler onClick={this.toggle}/>
          <Nav navbar={true}>
            <NavItem>
              <NavLink href="/signup">Sign up</NavLink>
            </NavItem>
            <NavItem>
              <NavLink href="/login">Log in</NavLink>
            </NavItem>
          </Nav>
      </StyledNavbar>
    )
  }

  private toggle() {
    this.setState({
      isOpen: !this.state.isOpen,
    })
  }
}
