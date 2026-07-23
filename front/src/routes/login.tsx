import { createFileRoute, redirect } from '@tanstack/react-router'
import { LoginComponent } from '../components/auth/Login'

export const Route = createFileRoute('/login')({
  component: RouteComponent,
  beforeLoad: async ({ context }) => {
    const me = context.client.getQueryData(["me"])
    if (me) throw redirect({ to: "/feed" })
  },
})


function RouteComponent() {
  return <LoginComponent />
}
