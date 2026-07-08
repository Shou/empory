import * as React from 'react'
import { createRootRoute } from '@tanstack/react-router'
import { Shell } from '../components/shell/Shell'

export const Route = createRootRoute({
  component: RootComponent,
})

function RootComponent() {
    return <Shell />
}
