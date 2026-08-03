import * as React from 'react'
import { createRootRouteWithContext, redirect } from '@tanstack/react-router'
import { Shell } from '../components/shell/Shell'
import type { QueryClient } from '@tanstack/react-query'
import * as ProfileAPI from '../api/profile'

export interface RouterContext {
  client: QueryClient,
}

export const Route = createRootRouteWithContext<RouterContext>()({
  beforeLoad: async ({ context, location }) => {
    console.log("Fetching metadata...")
    const me = await context.client.ensureQueryData(ProfileAPI.meQuery)

    if (me == null) {
      console.log("Metadata unavailable")
      if (location.pathname !== "/") throw redirect({ to: "/" })

    } else if (me.avatar_url == null) {
      console.log("Metadata unfinished, onboarding...")
      if (location.pathname !== "/onboarding") throw redirect({ to: "/onboarding" })
    }

    console.log("Metadata ready.")
  },
  component: RootComponent,
})

function RootComponent() {
    return <Shell />
}
