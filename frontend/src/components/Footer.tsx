import * as React from 'react'
import styled from 'styled-components'

const Wrapper = styled.div`
  text-align: center;
`

interface IProps {
  className?: string
}

const Footer = (props: IProps) => (
  <Wrapper className={props.className}>
    <small>
      Have an idea for a feature? Found a bug?&nbsp;
      <a href="https://github.com/twilco/cratify">
        Open an issue on Github!
      </a>
    </small>
  </Wrapper>
)

export default Footer
