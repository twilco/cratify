import * as fetchPonyfill from 'fetch-ponyfill'
const { fetch } = fetchPonyfill()

const isJson = (response: Response) => {
  return response.headers.get('content-type') === 'application/json'
}

const isText = (response: Response) => {
  return response.headers.get('content-type') === 'text/plain'
}

const handleResponse = (response: Response) => {
  if (response.ok) {
    if (isJson(response)) {
      return response.json().then((json) => {
        return {
          json,
          response,
        }
      })
    }
    if (isText(response)) {
      return response.text().then((text) => {
        return {
          response,
          text,
        }
      })
    }
    return Promise.resolve({ response })
  }
  return Promise.reject(response.statusText)
}

const request = (url: string, init: RequestInit = {}, json = true) => {
  const headers = json
    ? {
      headers: {
        'content-type': 'application/json',
      },
      ...init.headers,
    }
    : undefined
  const _init: RequestInit = {
    credentials: 'same-origin',
    ...init,
    ...headers,
  }
  return fetch(url, _init).then(handleResponse)
}

export default request
