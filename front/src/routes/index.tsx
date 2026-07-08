import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { Button } from '../components/ui/button'
import { RegisterComponent } from '../components/auth/Register'
import { useSelector } from '@tanstack/react-store'
import * as Auth from '../api/auth'

export const Route = createFileRoute('/')({
  component: RouteComponent,
})

function RouteComponent() {
  const navigate = useNavigate({ from: "/" })
  const token = useSelector(Auth.store, (state: Auth.Store) => state.token)

  if (token !== null) navigate({ to: "/feed" })

  return (
    <div className="p-px rounded-sm bg-linear-to-r from-white to-lime-50">
      <div className="flex flex-row w-2xl bg-white">
        <div className="w-64 bg-linear-to-r from-lime-600 to-lime-500 rounded-l-sm p-6">
          <div className="flex flex-col justify-between h-full bg-clip-text text-transparent bg-linear-to-r from-white via-lime-100 to-white font-extralight">
            <div>
              You are about to enter the shit. Prepare your mind.
            </div>
            <div className="text-left">
              You WILL enjoy:
              <ul className="flex flex-col items-start bg-clip-text text-transparent bg-linear-to-r from-white via-lime-100 to-white font-extralight">
                <li>💀 doom scrolling</li>
                <li>🙅‍♀️ unsolicited replies</li>
                <li>❄️ zero followers</li>
                <li>💣 manmade horrors</li>
              </ul>
            </div>
          </div>
        </div>
        <div className="flex flex-col gap-2 p-6 w-96">
          <RegisterComponent />
          <Button onClick={() => navigate({ to: "/login" })}>Login</Button>
        </div>
      </div>
    </div>
  )
  return <div>Hello "/"!</div>
}
