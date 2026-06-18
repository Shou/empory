
import { BASE_URL } from '../config'

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
    })
}