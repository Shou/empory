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

//type RegisterProps = null
export const RegisterComponent: React.FC = () => {
    const onSubmit = (event: React.SyntheticEvent<HTMLFormElement>) => {
        console.log("onSubmit")
        event.preventDefault()

        const formData = new FormData(event.currentTarget)
        const email = formData.get("regEmail")
        const username = formData.get("regUsername")
        const password = formData.get("regPassword")

        if (isFormDataString(email) && isFormDataString(username) && isFormDataString(password)) {
            API.sendRegister(email, username, password).then(resp => {
                console.log(resp)
            }).catch(err => {
                console.error(err)
            })
        } else {
            // throw error
        }
    }
    return (
        <form onSubmit={onSubmit}>
            <input name="regEmail" type="email" placeholder="Email" required />
            <input name="regUsername" type="text" placeholder="Username" required minLength={4} maxLength={32} />
            <input name="regPassword" type="password" placeholder="Password" required minLength={10} maxLength={128} />
            <button type="submit" value="Register">Register</button>
        </form>
    )
}