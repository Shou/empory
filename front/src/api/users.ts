
import { BASE_URL } from '../config'


export interface CreateFollow {
    user_id: string,
}
export interface Follow {
    user_id: string,
    followed_id: string,
    created_at: null,
}

export const followUser = async (token: string, user_id: string): Promise<Follow> => {
    const url = `${BASE_URL}/users/${user_id}/follow`
    const resp = await fetch(url, {
        method: "POST",
        headers: {
            "Authorization": "Bearer " + token,
        },
    })
    return resp.json()
}

export const unfollowUser = async (token: string, user_id: string): Promise<Follow> => {
    const url = `${BASE_URL}/users/${user_id}/follow`
    const resp = await fetch(url, {
        method: "DELETE",
        headers: {
            "Authorization": "Bearer " + token,
        },
    })
    return resp.json()
}