import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { Card, CardAction, CardContent, CardHeader, CardTitle } from '../components/ui/card'
import { Button } from '../components/ui/button'

import * as ProfileAPI from '../api/profile'
import * as Auth from '../api/auth'
import { useSelector } from '@tanstack/react-store'

export const Route = createFileRoute('/onboarding')({
  component: RouteComponent,
})

function RouteComponent() {
  const navigate = useNavigate({ from: "/onboarding" })
  const token = useSelector(Auth.store, (state: Auth.Store) => state.token)

  const uploadAvatar = (event: React.SyntheticEvent<HTMLFormElement>) => {
    console.log("uploadAvatar onSubmit")
    event.preventDefault()
    if (token === null) return
    const formData = new FormData(event.currentTarget)
    ProfileAPI.uploadAvatar(token, formData).then((response) => {
      navigate({ to: "/feed" })
    })
  }

  return (
    <form onSubmit={uploadAvatar}>
      <Card>
        <CardHeader>
          <CardTitle>Select avatar</CardTitle>
          <CardAction>
            <Button type="submit" disabled={token === null}>Submit</Button>
          </CardAction>
        </CardHeader>
        <CardContent>
          <input name="avatar" type="file" disabled={token === null} />
        </CardContent>
      </Card>
    </form>
  )
}
