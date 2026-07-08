import { BASE_URL } from '../config'
import { createStore } from '@tanstack/react-store'

export interface Store {
  token: string | null,
}
export const store = createStore<Store>({
  token: null,
})



export type RegisterUser = {
    email: string,
    username: string,
    password: string,
}

export type LoginUser = {
    username: string,
    password: string,
}

export const sendRegister = (email: string, username: string, password: string): Promise<Response> => {
    const url = BASE_URL + "/auth/register"
    const body: RegisterUser = {
        email,
        username,
        password,
    }

    return fetch(url, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(body),
    })
}

export const sendLogin = (username: string, password: string): Promise<Response> => {
    const url = BASE_URL + "/auth/login"
    const body: LoginUser = {
        username,
        password,
    }
    return fetch(url, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(body),
    }).then((res) => {
        if (res.ok) {
            res.json().then((json) => {
                store.setState((state: Store) => {
                    return { token: json.token }
                })
            })
        }
        return res
    })
}

export const sendLogout = (token: string) => {
    const url = BASE_URL + "/auth/logout"
    return fetch(url, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            "Authorization": "Bearer " + token,
        },
    }).then((res) => {
        if (res.ok) {
            res.json().then((json) => {
                store.setState((state: Store) => {
                    return { token: null }
                })
            })
        }
        return res
    })
}

export const getRefresh = async (): Promise<Response | null> => {
    const token = store.get().token
    console.log("getRefresh", token)
    const url = BASE_URL + "/auth/refresh"
    const res = await fetch(url, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
        },
    })
    if (res.ok) {
        res.json().then((json) => {
            console.log("tonken ", token)
            store.setState((state: Store) => {
                return { token: json.token }
            })
        })
    }
    return res
}
