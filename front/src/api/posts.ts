
import { infiniteQueryOptions } from '@tanstack/react-query'
import { BASE_URL } from '../config'
import { getToken } from '../auth/tokenManager'


export interface Post {
    id: number
    user_id: string
    content: string
    created_at: string
}

export interface PostsQuery {
    before_at: Date | null
    before_post_id: string | null
    limit: number | null
}

interface CreatePost {
    content: string
}

export const allPostsQuery = infiniteQueryOptions({
    queryKey: ["posts", "suggested"],
    //staleTime: 10 * 60 * 1000,
    queryFn: async (context) => {
        const token = await getToken()

        const page: [Date, string] | undefined = context.pageParam
        const params = new URLSearchParams()
        if (page !== undefined) {
            params.append("before_at", page[0].toISOString())
            params.append("before_post_id", page[1])
        }
        const url = `${BASE_URL}/posts?${params.toString()}`
        const resp = await fetch(url, {
            // TODO use QUERY method in the future...
            method: "GET",
            headers: {
                "Content-Type": "application/json",
                "Authorization": "Bearer " + token,
            },
        })
        const json = await resp.json()
        console.log(json, "allPostQuery json")
        return json
    },
    initialPageParam: undefined,
    getNextPageParam: (posts: Array<Post>): [Date, string] | undefined => {
        const lastPost = posts.at(-1)
        if (lastPost === undefined) return undefined
        return [
            new Date(lastPost.created_at),
            Math.trunc(lastPost.id).toString(),
        ]
    },
})

export const feedQuery = infiniteQueryOptions({
    queryKey: ["posts", "following"],
    //staleTime: 10 * 60 * 1000,
    queryFn: async (context) => {
        const token = await getToken()

        const page: [Date, string] | undefined = context.pageParam
        const params = new URLSearchParams()
        if (page !== undefined) {
            params.append("before_at", page[0].toISOString())
            params.append("before_post_id", page[1])
        }
        const url = `${BASE_URL}/feed?${params.toString()}`
        const resp = await fetch(url, {
            // TODO use QUERY method in the future...
            method: "GET",
            headers: {
                "Content-Type": "application/json",
                "Authorization": "Bearer " + token,
            },
        })
        return resp.json()
    },
    initialPageParam: undefined,
    getNextPageParam: (posts: Array<Post>): [Date, string] | undefined => {
        const lastPost = posts.at(-1)
        if (lastPost === undefined) return undefined
        return [
            new Date(lastPost.created_at),
            Math.trunc(lastPost.id).toString(),
        ]
    },
})

export const createPost = async (token: string, content: string): Promise<Post> => {
    const url = BASE_URL + "/posts"
    const response = await fetch(url, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            "Authorization": "Bearer " + token,
        },
        body: JSON.stringify({
            content,
        } satisfies CreatePost),
    })
    return response.json()
}