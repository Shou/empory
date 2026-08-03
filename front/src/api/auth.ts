import { mutationOptions } from '@tanstack/react-query'
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

export const registerMutation = mutationOptions({
    mutationFn: async ({ email, username, password }: RegisterUser) => {
        const url = BASE_URL + "/auth/register"
        const body: RegisterUser = {
            email,
            username,
            password,
        }
        const response = await fetch(url, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(body),
        })
        return response.json()
    },
})

export const loginMutation = mutationOptions({
    mutationFn: async ({ username, password }: LoginUser) => {
        const url = BASE_URL + "/auth/login"
        const body: LoginUser = {
            username,
            password,
        }
        const response = await fetch(url, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(body),
        })
        return response.json()
    },
})

export const logoutMutation = mutationOptions({
    mutationFn: async () => {
        const url = BASE_URL + "/auth/logout"
        return fetch(url, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
        })
    },
})