
import { BASE_URL } from '../config'


interface Avatar {
    avatar_url: string,
}


export const getAvatar = async (token: string): Promise<Avatar> => {
    const url = BASE_URL + "/profile/avatar"
    const response = await fetch(url, {
        method: "GET",
        headers: {
            "Authorization": "Bearer " + token,
        },
    })
    return response.json()
}

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