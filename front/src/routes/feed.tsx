import * as React from 'react'
import { createFileRoute, redirect } from '@tanstack/react-router'
import { useInfiniteQuery, useQuery } from '@tanstack/react-query'
import * as PostsAPI from '../api/posts'
import { Spinner } from '../components/ui/spinner'
import { Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '../components/ui/card'
import { getRelativeTime, isFormDataString } from '../lib/utils'
import { Button } from '../components/ui/button'

import * as UsersAPI from '../api/users'
import { getToken, useToken } from '../auth/tokenManager'
import * as Profile from '../api/profile'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '../components/ui/tabs'

type Status = "onboarding"
interface MeStatus {
  status: Status,
  avatar_url: string,
}

export const Route = createFileRoute('/feed')({
  component: RouteComponent,
  beforeLoad: async ({ context }) => {
    const me: MeStatus | undefined = context.client.getQueryData(["me"])
    if (!me) throw redirect({ to: "/" })
    else if (me.status === "onboarding") throw redirect({ to: "/onboarding" })
  },
})

function RouteComponent() {
  const { data: me } = useQuery(Profile.meQuery)
  const [tab, setTab] = React.useState<"following" | "suggested">("following")
  const { data, error, isLoading, isError, fetchNextPage } = useInfiniteQuery({
    ...(tab === "following" ? PostsAPI.allPostsQuery : PostsAPI.feedQuery),
  })
  const [isPosting, startTransition] = React.useTransition()

  console.log(data, "data as")
  console.log(error, "error")

  // this should be on each tweet card instead maybe...
  const sendFollow = async (user_id: string) => {
    const token = await getToken()
    UsersAPI.followUser(token, user_id)
  }
  const sendPost = (event: React.SyntheticEvent<HTMLFormElement>) => {
    console.log("sendPost")
    event.preventDefault()

    const formData = new FormData(event.currentTarget)
    const content = formData.get("content")

    if (!isFormDataString(content)) return

    startTransition(async () => {
      const token = await getToken()
      PostsAPI.createPost(token, content).then(() => {
        console.log("we postin shit")
        // TODO refresh the posts view or smth...
      })
    })
  }

  const posts = data?.pages.flatMap(page => page) ?? []
  const postElems = posts.map((post: PostsAPI.Post, ix: number) => {
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

  let elem = null
  if (isLoading) elem = <Spinner />
  else if (isError) elem = <>Error: {JSON.stringify(error)}</>
  else if (data === undefined) elem = <>no data</>
  else elem = postElems

  return (
    <div className="flex flex-col gap-6">
      <Tabs value={tab} onValueChange={tab => setTab(tab)}>
        <TabsList>
          <TabsTrigger value="suggested">Suggested</TabsTrigger>
          <TabsTrigger value="following">Following</TabsTrigger>
        </TabsList>
        <TabsContent value="suggested">
        </TabsContent>
        <TabsContent value="following">
        </TabsContent>
      </Tabs>
      <Card>
        <CardHeader>
          <CardTitle>{me.username}</CardTitle>
          <CardAction>
            <img src={"/files" + me?.avatar_url} className='w-6 h-6' />
          </CardAction>
        </CardHeader>
        <CardContent>
          <form onSubmit={sendPost} className="flex flex-col">
            <textarea name="content" placeholder="Post something new..." />
            {
              isPosting ? (
                <Spinner />
              ) : (
                <Button type="submit">Submit</Button>
              )
            }
          </form>
        </CardContent>
      </Card>
      {elem}
    </div>
  )
}
