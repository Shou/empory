import * as React from 'react'
import { createFileRoute } from '@tanstack/react-router'
import { useQuery, useQueryClient } from '@tanstack/react-query'
import * as PostsAPI from '../api/posts'
import * as Auth from '../api/auth'
import { useSelector } from '@tanstack/react-store'
import { Spinner } from '../components/ui/spinner'
import { Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '../components/ui/card'
import { getRelativeTime, isFormDataString } from '../lib/utils'
import { Button } from '../components/ui/button'

import * as UsersAPI from '../api/users'

export const Route = createFileRoute('/feed')({
  component: RouteComponent,
})

function RouteComponent() {
  const qclient = useQueryClient()
  const token = useSelector(Auth.store, (state: Auth.Store) => state.token)
  const { data, error, isLoading, isError } = useQuery({
    queryKey: ["posts"],
    // NOTE token won't be null here because of the enabled field
    // and we can't put React hooks behind if-statements
    queryFn: () => PostsAPI.getAllPosts(token!, new Date()),
    enabled: token !== null,
  })

  if (token === null) return <Spinner />

  console.log(data)

  if (isLoading) return <>LOADING</>
  if (isError) return <>{JSON.stringify(error)}</>
  if (data === undefined) return <>wtf</>

  const sendFollow = (user_id: string) => {
    UsersAPI.followUser(token, user_id)
  }
  const sendPost = (event: React.SyntheticEvent<HTMLFormElement>) => {
    console.log("sendPost")
    event.preventDefault()

    const formData = new FormData(event.currentTarget)
    const content = formData.get("content")

    if (isFormDataString(content)) {
      PostsAPI.createPost(token, content).then(() => {
        console.log("we postin shit")
        // TODO in the future we'll use materialized views so this won't immediately be available
        // instead we should query for _this user's specific tweets_ as well + combine?
        qclient.invalidateQueries({ queryKey: ["posts"] })
      })
    }
  }

  const posts = data.map((post: PostsAPI.Post, ix: number) => {
    const relTime = getRelativeTime(new Date(post.created_at))
    return (
      <Card key={ix}>
        <CardHeader>
          <CardTitle>
            {post.user_id.substring(0, 8)}
          </CardTitle>
          <CardDescription>{relTime}</CardDescription>
          <CardAction>
            <Button onClick={() => sendFollow(post.user_id)}>+ follow</Button>
          </CardAction>
        </CardHeader>
        <CardContent>
          {post.content}
        </CardContent>
        <CardFooter>
          <Button variant="secondary">+2</Button>
        </CardFooter>
      </Card>
    )
  })
  return (
    <div className="flex flex-col gap-6">
      <Card>
        <CardHeader>
          <CardTitle>Create new post</CardTitle>
        </CardHeader>
        <CardContent>
          <form onSubmit={sendPost} className="flex flex-col">
            <textarea name="content" />
            <Button type="submit">Submit</Button>
          </form>
        </CardContent>
      </Card>
      {posts}
    </div>
  )
}
