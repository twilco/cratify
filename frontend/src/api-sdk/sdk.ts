// Polyfill for fetch
import 'whatwg-fetch'
import wretch from 'wretch'

interface IAvailable {
  available: boolean,
}

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

export const usernameAvailable = async (username: string) => {
  try {
    const resp: IAvailable = await wretch('/api/available').post({
      username,
    }).json()
    return resp.available
  } catch (e) {
    console.error(`error attempting to determine if username available: ${e}`)
    return { error: e }
  }
}
