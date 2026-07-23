import * as React from 'react'
import { createRootRouteWithContext } from '@tanstack/react-router'
import { Shell } from '../components/shell/Shell'
import type { QueryClient } from '@tanstack/react-query'
import * as ProfileAPI from '../api/profile'
import { getToken } from '../auth/tokenManager'

export interface RouterContext {
  client: QueryClient,
}

export const Route = createRootRouteWithContext<RouterContext>()({
  beforeLoad: async ({ context }) => {
    console.log("Fetching metadata...")
    await getToken()
    await context.client.ensureQueryData(ProfileAPI.meQuery)
    console.log("Metadata ready.")
  },
  component: RootComponent,
})

function RootComponent() {
    return <Shell />
}
