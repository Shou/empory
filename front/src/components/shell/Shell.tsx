import * as React from "react"
import { Link, Outlet } from "@tanstack/react-router"
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools"
import * as ProfileAPI from "../../api/profile"
import { useQuery } from "@tanstack/react-query"


function HeaderStatus() {
  const { data: me, isLoadingError } = useQuery(ProfileAPI.meQuery)
  if (isLoadingError || !me) return <>not logged in</>
  return (
    <div className="flex flex-row">
      {me.username}
      <img src={"/files" + me.avatar_url} className="w-8 h-8" />
    </div>
  )
}

export function Header() {
  return (
    <div className="flex flex-row justify-between">
      <Link to="/">
        <h2 className="inline-flex p-2 text-transparent bg-clip-text font-bold bg-linear-to-r from-[#e3f1bd] via-[#f0f346] to-[#e3f1bd]">
          EMPORY
        </h2>
      </Link>
      <HeaderStatus />
    </div>
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
          © Empory {(new Date).getFullYear()}
        </div>
      </div>
    </footer>
  )
}

export type ShellProps = {
    children: React.ReactNode,
}
export const Shell: React.FC = () => {
  return (
    <>
      <div className="grid grid-cols-1 content-between min-h-screen bg-linear-to-r from-[#c2dd05] via-[#96b203] to-[#c2dd05]">
          <Header />
          <main className="flex justify-center m-8">
            <Outlet />
          </main>
          <Footer />
      </div>
      <TanStackRouterDevtools position="bottom-right" />
    </>
  )
}
