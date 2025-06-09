const TOKEN_NAME = 'X-ATOMGIT-TOKEN'

export const saveToken = (token: string) => {
  localStorage.setItem(TOKEN_NAME, token)
}

export const getToken = () => {
  const token = localStorage.getItem(TOKEN_NAME)
  return token || null
}

export const delToken = () => {
  localStorage.removeItem(TOKEN_NAME)
}
