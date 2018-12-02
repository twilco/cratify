import { TranslationFunction } from 'i18next'
import * as React from 'react'
import { Trans } from 'react-i18next'
import { Link } from 'react-router-dom'
import { Button, Jumbotron } from 'reactstrap'
import styled from 'styled-components'

const Infotron = styled(Jumbotron)`
  margin-bottom: 0 !important;
`

interface IProps {
  t: TranslationFunction
}

const About = ({ t }: IProps) => {
  return (
    <div className="container">
      <div className="row justify-content-md-center">
        <div className="col-9">
          <Infotron>
            <h1 className="display-3">{ t('welcome-to-cratify') }</h1>
            <p className="lead">
              { t('simple-short-hook') }
            </p>
            <hr/>
            <p>
              <Trans i18nKey="how-it-works">
                <a href="https://crates.io/">crates.io</a>
                <span className="font-italic">cratify</span>
              </Trans>
            </p>
            <h3>{ t('how-can-i-use-it') }</h3>
            <p>
              <Trans i18nKey="usage-example">
                <code>coolcrate</code>
                <code>coolcrate v0.1.0</code>
                <code>coolcrate v0.2.0</code>
                <em>{ t('any') }</em>
              </Trans>
            </p>
            <p>
              <Trans i18nKey="sub-types-desc">
                <strong>{ t('immediate') }</strong>
                <strong>{ t('periodic') } </strong>
              </Trans>
            </p>
            <h3>{ t('sub-types-and-fulfillment-header') }</h3>
            <p>
              { t('fulfillment-desc') }
            </p>
            <p className="lead">
              { t('get-started-click') }
              <br/>
              <Link to="/signup">
                <Button color="primary">{ t('sign-up') }</Button>
              </Link>
            </p>
          </Infotron>
        </div>
      </div>
    </div>
  )
}

export default About
