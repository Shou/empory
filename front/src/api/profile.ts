import { queryOptions } from '@tanstack/react-query'
import { BASE_URL } from '../config'
import { getToken } from '../auth/tokenManager'


interface Avatar {
    avatar_url: string,
}


export const avatarQuery = (userId: String) => queryOptions({
    queryKey: ["avatar", userId],
    queryFn: async (context) => {
        const token = await getToken()
        const url = BASE_URL + "/profile/avatar"
        const response = await fetch(url, {
            method: "GET",
            headers: {
                "Authorization": "Bearer " + token,
            },
        })
        return response.json()
    }
})

export const meQuery = queryOptions({
    queryKey: ["me"],
    queryFn: async (context) => {
        const token = await getToken()
        const url = BASE_URL + "/me"
        const response = await fetch(url, {
            method: "GET",
            headers: {
                "Authorization": "Bearer " + token,
            },
        })
        return response.json()
    },
})

// TODO mutation
export const uploadAvatar = async (token: string, body: FormData): Promise<Avatar> => {
    const url = BASE_URL + "/profile/avatar"
    const response = await fetch(url, {
        method: "POST",
        headers: {
            "Authorization": "Bearer " + token,
        },
        body,
    })
    return response.json()
}