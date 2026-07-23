import { createFileRoute } from '@tanstack/react-router'
import { RegisterComponent } from '../components/auth/Register'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '../components/ui/tabs'
import { LoginComponent } from '../components/auth/Login'

export const Route = createFileRoute('/')({
  component: RouteComponent,
})

function RouteComponent() {
  return (
    <div className="p-px rounded-sm bg-linear-to-r from-white to-lime-50">
      <div className="flex flex-row w-2xl bg-white">
        <aside className="w-64 bg-linear-to-r from-lime-600 to-lime-500 rounded-l-sm p-6">
          <div className="flex flex-col justify-between h-full bg-clip-text text-transparent bg-linear-to-r from-white via-lime-100 to-white font-extralight">
            <h6>
              You are about to enter the shit. Prepare your mind.
            </h6>
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
        </aside>
        <div className="flex flex-col gap-2 p-6 w-96">
          <Tabs defaultValue="register">
            <TabsList>
              <TabsTrigger value="register">Register</TabsTrigger>
              <TabsTrigger value="login">Login</TabsTrigger>
            </TabsList>
            <TabsContent value="register">
              <RegisterComponent />
            </TabsContent>
            <TabsContent value="login">
              <LoginComponent />
            </TabsContent>
          </Tabs>
        </div>
      </div>
    </div>
  )
}