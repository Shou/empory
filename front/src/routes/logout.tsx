import * as React from 'react'
import { createFileRoute, useNavigate } from '@tanstack/react-router'
import * as Auth from '../api/auth'
import { Spinner } from '../components/ui/spinner'
import { useMutation, useQueryClient } from '@tanstack/react-query'

export const Route = createFileRoute('/logout')({
  component: RouteComponent,
})


function RouteComponent() {
  const queryClient = useQueryClient()
  const navigate = useNavigate({ from: "/logout" })
  const logout = useMutation({
    ...Auth.logoutMutation,
    onSuccess: () => {
      queryClient.removeQueries({ queryKey: ["refresh"] })
      queryClient.removeQueries({ queryKey: ["me"] })
      navigate({ to: "/" })
    },
  })

  if (logout.isPending) return <Spinner />

  return null
}