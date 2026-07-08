import * as React from 'react'
import { createFileRoute, useNavigate } from '@tanstack/react-router'
import * as Auth from '../api/auth'
import { Spinner } from '../components/ui/spinner'
import { useSelector } from '@tanstack/react-store'

export const Route = createFileRoute('/logout')({
  component: RouteComponent,
})


export function RouteComponent() {
  const navigate = useNavigate({ from: "/logout" })
  const token = useSelector(Auth.store, (state: Auth.Store) => state.token)

  React.useEffect(() => {
    if (token === null) return
    Auth.sendLogout(token).then(resp => {
      console.log(resp)
      navigate({ to: "/" })
    })
  }, [token])

  return (
    <Spinner />
  )
}