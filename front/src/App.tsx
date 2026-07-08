import * as React from 'react'
import './App.css'
import {
  RouterProvider,
} from '@tanstack/react-router'
import { routeTree } from './routeTree.gen'
import { createRouter } from '@tanstack/react-router'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { createStore } from '@tanstack/react-store'



const router = createRouter({
  routeTree,
  defaultPreload: 'intent',
  scrollRestoration: true,
})

const queryClient = new QueryClient()

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <RouterProvider router={router} />
    </QueryClientProvider>
  )
}



export default App
