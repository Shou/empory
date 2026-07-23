import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { Card, CardAction, CardContent, CardHeader, CardTitle } from '../components/ui/card'
import { Button } from '../components/ui/button'
import * as ProfileAPI from '../api/profile'
import { getToken, useToken } from '../auth/tokenManager'
import { useTransition } from 'react'
import { Spinner } from '../components/ui/spinner'

export const Route = createFileRoute('/onboarding')({
  component: RouteComponent,
})

function RouteComponent() {
  const navigate = useNavigate({ from: "/onboarding" })
  const [isPending, startTransition] = useTransition()

  const uploadAvatar = (event: React.SyntheticEvent<HTMLFormElement>) => {
    console.log("uploadAvatar onSubmit")
    event.preventDefault()
    const elem = event.currentTarget
    startTransition(async () => {
      const token = await getToken()
      if (token === null) return
      const formData = new FormData(elem)
      const url = await ProfileAPI.uploadAvatar(token, formData)
      navigate({ to: "/feed" })
      console.log("Avatar uploaded = ", url)
    })
  }

  return (
    <form onSubmit={uploadAvatar}>
      <Card>
        <CardHeader>
          <CardTitle>Select avatar</CardTitle>
          <CardAction>
            {
              isPending ? (
                <Spinner />
              ) : (
                <Button type="submit">Submit</Button>
              )
            }
          </CardAction>
        </CardHeader>
        <CardContent>
          <input name="avatar" type="file" />
        </CardContent>
      </Card>
    </form>
  )
}
