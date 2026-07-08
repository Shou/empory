import React from 'react'
import { createFileRoute } from '@tanstack/react-router'
import { RegisterComponent } from '../components/auth/Register'

export const Route = createFileRoute('/register')({
  component: RouteComponent,
})

// TODO make this a full ass page and not just a form...
function RouteComponent() {
  return <RegisterComponent />
}