// Polyfill for fetch
import 'whatwg-fetch'
import wretch from 'wretch'

export const signup = async (username: string, password: string) => {
  try {
    return await wretch('/api/signup').post({
      password,
      username,
    }).json()
  } catch (e) {
    console.error(`error attempting to sign up: ${e}`)
    return { error: e }
  }
}
