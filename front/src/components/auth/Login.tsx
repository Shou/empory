import { useNavigate } from '@tanstack/react-router'
import * as API from '../../api/auth'
import { Input } from '../../components/ui/input'
import { Button } from '../../components/ui/button'
import { isFormDataString } from '../../lib/utils'
import { useMutation, useQueryClient } from '@tanstack/react-query'


export function LoginComponent() {
  const queryClient = useQueryClient()
  const navigate = useNavigate({ from: "/login" })
  const login = useMutation({
    ...API.loginMutation,
    onSuccess: async (json) => {
      queryClient.setQueryData(["refresh"], json)
      await navigate({ to: "/feed" })
    },
  })

  const onSubmit = (event: React.SyntheticEvent<HTMLFormElement>) => {
    console.log("onSubmit")
    event.preventDefault()

    const formData = new FormData(event.currentTarget)
    const username = formData.get("loginUsername")
    const password = formData.get("loginPassword")

    if (isFormDataString(username) && isFormDataString(password)) {
      login.mutate({ username, password })
    }
  }

  return (
    <form onSubmit={onSubmit}>
      <Input name="loginUsername" type="text" placeholder="Username" required minLength={4} maxLength={32} />
      <Input name="loginPassword" type="password" placeholder="Password" required minLength={10} maxLength={128} />
      <Button type="submit" value="Login">Login</Button>
    </form>
  )
}
