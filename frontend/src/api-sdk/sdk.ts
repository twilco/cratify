// Polyfill for fetch
import 'whatwg-fetch'
import wretch from 'wretch'

export const login = async (username: string, password: string) => {
  return await wretch('/api/login').post({
    password,
    username,
  }).res()
}

export const signup = async (username: string, password: string) => {
  return await wretch('/api/signup').post({
    password,
    username,
  }).res()
}

export const usernameAvailable = async (username: string) => {
  const resp = await wretch('/api/available').post({
    username,
  }).json()
  return resp.available
}

/**
 * Parses error resulting from failed request to server into a useful error string.
 *
 * When our server responds to a request with an error, it sends a JSON payload containing an error message in a `msg`
 * field.  We attempt to parse that message as a first step, then fallback to stringifying the error itself, finally
 * falling back to an empty string if all else fails.
 * @param err
 */
export const extractErrMessage = (err: any) => {
  return err && (JSON.parse(err.message).msg || err.toString()) || ''
}
