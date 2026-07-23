import * as React from 'react'
import './App.css'
import {
  RouterProvider,
} from '@tanstack/react-router'
import { routeTree } from './routeTree.gen'
import { createRouter } from '@tanstack/react-router'
import { QueryClientProvider } from '@tanstack/react-query'
import { queryClient } from './auth/tokenManager'




const router = createRouter({
  routeTree,
  defaultPreload: 'intent',
  scrollRestoration: true,
  context: {
    client: queryClient,
  }
})

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <RouterProvider router={router} />
    </QueryClientProvider>
  )
}



export default App
