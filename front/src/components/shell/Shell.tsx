import * as React from "react"
import { Link, Outlet, useNavigate } from "@tanstack/react-router"
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools"
import * as Auth from '../../api/auth'
import { Spinner } from "../ui/spinner"
import { useSelector } from "@tanstack/react-store"


export function Header() {
  return (
    <Link to="/">
      <h1 className="inline-flex p-2 text-transparent bg-clip-text font-bold bg-linear-to-r from-[#e3f1bd] via-[#f0f346] to-[#e3f1bd]">
        BIRDSHIT
      </h1>
    </Link>
  )
}

export function Footer() {
  return (
    <footer className="font-extralight text-sm bg-linear-to-r from-mauve-700 via-mauve-900 to-mauve-700 border-t-2 border-t-lime-100 p-1.5">
      <div className="flex justify-between text-transparent bg-clip-text bg-linear-to-r from-mauve-200 via-mauve-100 to-mauve-200">
        <div>
          Made without ☕ (truly evil)
        </div>
        <div>
          © Birdshit {(new Date).getFullYear()}
        </div>
      </div>
    </footer>
  )
}

export type ShellProps = {
    children: React.ReactNode,
}
export const Shell: React.FC = () => {
  const navigate = useNavigate()
  const token = useSelector(Auth.store, (state: Auth.Store) => state.token)

  React.useEffect(() => {
    console.log("wtf")
    Auth.getRefresh().then((resp: Response | null) => {
      if (resp === null || !resp.ok) navigate({ to: "/" })
    })
  }, [])

  if (token === null) {
    return (
      <Spinner />
    )
  }

  return (
    <>
      <div className="grid grid-cols-1 content-between min-h-screen bg-linear-to-r from-[#c2dd05] via-[#96b203] to-[#c2dd05]">
          <Header />
          <main className="flex justify-center">
              <Outlet />
          </main>
          <Footer />
      </div>
      <TanStackRouterDevtools position="bottom-right" />
    </>
  )
}