
import { useNavigate } from '@tanstack/react-router'
import * as Auth from '../../api/auth'
import { Button } from '../ui/button'
import { Input } from '../ui/input'
import { isFormDataString } from '../../lib/utils'
import { useMutation, useQueryClient } from '@tanstack/react-query'

export function RegisterComponent() {
  const queryClient = useQueryClient()
  const navigate = useNavigate({ from: "/register" })
  const register = useMutation({
    ...Auth.registerMutation,
    onSuccess: async (json) => {
      queryClient.setQueryData(["refresh"], json)
      await navigate({ to: "/onboarding" })
    },
  })
  const onSubmit = (event: React.SyntheticEvent<HTMLFormElement>) => {
    console.log("onSubmit")
    event.preventDefault()

    const formData = new FormData(event.currentTarget)
    const email = formData.get("regEmail")
    const username = formData.get("regUsername")
    const password = formData.get("regPassword")

    if (isFormDataString(email) && isFormDataString(username) && isFormDataString(password)) {
      register.mutate({ email, username, password })
    }
  }
  return (
    <form onSubmit={onSubmit} className="flex flex-col gap-3">
      <Input name="regEmail" type="email" placeholder="Email" required />
      <Input name="regUsername" type="text" placeholder="Username" required minLength={4} maxLength={32} />
      <Input name="regPassword" type="password" placeholder="Password" required minLength={10} maxLength={128} />
      <Button type="submit" value="Register">Register</Button>
    </form>
  )
}