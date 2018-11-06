import * as React from 'react'
import { Button, Jumbotron } from 'reactstrap'

const About = () => {
  return (
    <div className="container">
      <div className="row justify-content-md-center">
        <div className="col-9">
          <Jumbotron>
            <h1 className="display-3">Welcome to Cratify</h1>
            <p className="lead">
              Ever want to know when another crate depends on yours?  We got you covered!
            </p>
            <hr/>
            <p>
              Cratify is simple.  Sign up to receive notifications for a crate name and version and we'll watch
              the <a href="https://crates.io/">crates.io</a> index for you.  When something interesting happens, we will
              notify (<span className="font-italic">cratify</span>) you!
            </p>
            <h3>How can I use it?</h3>
            <p>
              There are lots of neat ways you can use Cratify. Perhaps you're releasing a new version of your crate,
              let's call it <code>coolcrate</code>, and want to measure adoption of this new version versus the old.
              Create two subscriptions, one for <code>coolcrate v0.1.0</code> and one
              for <code>coolcrate v0.2.0</code>, and Cratify will let you know when each one is depended upon.
              Or perhaps you're just curious and want to know when another crate uses <em>any</em> version of your
              crate - Cratify can handle that, too.
            </p>
            <h3>Subscription types and fulfillment</h3>
            <p>
              There are a couple subscription types you can choose from.  The first is called
              an <strong>immediate</strong> subscription.  As you might expect, when you create
              an <strong>immediate</strong> subscription we will notify you immediately when a new crate depends on the
              crate name and version you specify.
            </p>
            <p>
              The next type of subscription is a <strong>periodic</strong> subscription.  With
              a <strong>periodic</strong> subscription, we will send you a weekly, monthly, or yearly summary of all
              crates that depend on yours.
            </p>
            <p>
              Cratify allows you to specify any number of e-mail addresses to fulfill your subscriptions with.  Want
              to be notified on your personal e-mail for certain crates, and your work e-mail for others?  You can do
              that!
            </p>
            <p className="lead">
              Sound good?  Click below to get started. <br/>
              <Button color="primary">Sign up</Button>
            </p>
          </Jumbotron>
        </div>
      </div>
    </div>
  )
}

export default About
