import request from './request'

export const signup = async (username: string, password: string) => {
  try {
    const resp = await request('/signup', { body: JSON.stringify({
      password,
      username,
    }), method: 'POST' })
    return resp.response.json()
  } catch (e) {
    console.error(`error attempting to sign up: ${e}`)
  }
}
