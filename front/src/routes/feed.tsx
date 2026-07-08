import * as React from 'react'
import { createFileRoute } from '@tanstack/react-router'
import { useQuery } from '@tanstack/react-query'
import { getPosts, type Post } from '../api/posts'
import * as Auth from '../api/auth'
import { useSelector } from '@tanstack/react-store'
import { Spinner } from '../components/ui/spinner'

export const Route = createFileRoute('/feed')({
  component: RouteComponent,
})

function RouteComponent() {
  const token = useSelector(Auth.store, (state: Auth.Store) => state.token)

  if (token === null) return <Spinner />

  const { data, error, isLoading, isError } = useQuery({
    queryKey: ["posts"],
    queryFn: () => getPosts(token),
  })

  console.log(data)

  if (isLoading) return <>LOADING</>
  if (isError) return <>{JSON.stringify(error)}</>
  if (data === undefined) return <>wtf</>

  const posts = data.map((post: Post) => {
    return <div>{post.user_id} - {post.content}</div>
  })
  return (
    <div className="flex flex-col">
      {posts}
    </div>
  )
}
