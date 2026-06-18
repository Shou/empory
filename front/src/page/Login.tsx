"use client"

import React from 'react'
//import * as API from '@/api/auth'
import * as API from '../api/auth'


type ShellProps = {
    children: React.ReactNode,
}
export const Shell: React.FC<ShellProps> = (props) => {
    return (
        <div className="grid grid-cols-1 content-between min-h-max">
            <header>BIRDSHIT</header>
            <main>
                { props.children }
            </main>
            <footer>© Birdshit 2026</footer>
        </div>
    )
}


const isFormDataString = (fd: FormDataEntryValue | null): fd is string => {
    return typeof fd === "string"
}

export const LoginComponent: React.FC = () => {
    const onSubmit = (event: React.SyntheticEvent<HTMLFormElement>) => {
        console.log("onSubmit")
        event.preventDefault()

        const formData = new FormData(event.currentTarget)
        const email = formData.get("loginEmail")
        const username = formData.get("loginUsername")
        const password = formData.get("loginPassword")

        if (isFormDataString(email) && isFormDataString(username) && isFormDataString(password)) {
            API.sendLogin(username, password).then(resp => {
                console.log(resp)
            })
        } else {
            // throw error
        }
    }
    return (
        <form onSubmit={onSubmit}>
            <input name="loginEmail" type="email" placeholder="Email" required />
            <input name="loginUsername" type="text" placeholder="Username" required minLength={4} maxLength={32} />
            <input name="loginPassword" type="password" placeholder="Password" required minLength={10} maxLength={128} />
            <button type="submit" value="Login">Login</button>
        </form>
    )
}