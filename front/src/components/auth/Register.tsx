
import { useNavigate } from '@tanstack/react-router'
import * as Auth from '../../api/auth'
import { Button } from '../ui/button'
import { Input } from '../ui/input'
import { isFormDataString } from '../../lib/utils'

export function RegisterComponent() {
  const navigate = useNavigate({ from: "/login" })
  const onSubmit = (event: React.SyntheticEvent<HTMLFormElement>) => {
    console.log("onSubmit")
    event.preventDefault()

    const formData = new FormData(event.currentTarget)
    const email = formData.get("regEmail")
    const username = formData.get("regUsername")
    const password = formData.get("regPassword")

    if (isFormDataString(email) && isFormDataString(username) && isFormDataString(password)) {
      Auth.sendRegister(email, username, password).then(resp => {
          console.log(resp)
          navigate({ to: "/feed" })
      }).catch((err: Error) => {
          console.error(err)
      })
    } else {
      // throw error
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