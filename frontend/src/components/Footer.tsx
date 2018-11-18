import { TranslationFunction } from 'i18next'
import * as React from 'react'
import styled from 'styled-components'

const Wrapper = styled.div`
  text-align: center;
`

interface IProps {
  className?: string,
  t: TranslationFunction
}

const Footer = ({ className, t }: IProps) => (
  <Wrapper className={className}>
    <small>
      { t('bug-or-feature') }&nbsp;
      <a href="https://github.com/twilco/cratify">
        { t('open-issue') }
      </a>
    </small>
  </Wrapper>
)

export default Footer
