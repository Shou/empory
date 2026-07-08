
import { BASE_URL } from '../config'


export interface Post {
    id: number,
    user_id: string,
    content: string,
    created_at: string,
}

interface CreatePost {
    content: string,
}

export const getPosts = async (token: string): Promise<Array<Post>> => {
    const url = BASE_URL + "/posts"
    const resp = await fetch(url, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            "Authorization": "Bearer " + token,
        },
    })
    return resp.json()
}

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