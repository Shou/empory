
import '@empory/config'
import { it, expect } from 'vitest'
import newman from 'newman'
import { collection as authCollection } from './collections/auth.collection'
import { collection as postsCollection } from './collections/posts.collection'

const BASE_URL = `http://${process.env.BACK_HOST}:${process.env.BACK_PORT}`

const environment = {
    values: [
        { key: "baseUrl", value: BASE_URL },
        { key: "authUsername", value: `newUsername${Math.random().toString(36).substring(2, 10)}` },
        { key: "authPassword", value: "test test test" },
        { key: "authWrongname", value: `newUsername${Math.random().toString(36).substring(2, 10)}` },
        { key: "post", value: `post${Math.random().toString(36).substring(2, 10)}` },
    ],
} satisfies newman.NewmanRunOptions["environment"]

let token: string

it("/auth", async () => {
    const summary: newman.NewmanRunSummary = await new Promise((resolve, reject) => {
        newman.run(
            {
                collection: authCollection,
                environment,
                reporters: ["cli"],
            },
            (err, summary1) => {
                if (err) reject(err)
                resolve(summary1)
            }
        )
    })

    expect(summary.run.failures).to.toHaveLength(0)
    summary.run.executions.forEach(pmex => {
        switch (pmex.item.name) {
            case "Login WRONG user": {
                expect(pmex.response.code).toBe(401)
                break
            }
            case "Login WRONG password": {
                expect(pmex.response.code).toBe(401)
                break
            }
            case "Login user": {
                token = pmex.response.json().token
                console.log("TOKEN", token)
                expect(pmex.response.code).toBe(200)
                break
            }
            default: {
                expect(pmex.response.code).toBe(200)
                break
            }
        }
    })
})

it("/posts", async () => {
    environment.values?.push({
        key: "accessToken",
        value: token,
    })
    console.log(environment)
    const summary: newman.NewmanRunSummary = await new Promise((resolve, reject) => {
        newman.run(
            {
                collection: postsCollection,
                environment,
                reporters: ["cli"],
            },
            (err, summary1) => {
                if (err) reject(err)
                resolve(summary1)
            }
        )
    })

    let post: null | { id: number, user_id: string, content: string, created_at: string } = null
    expect(summary.run.failures).to.toHaveLength(0)
    summary.run.executions.forEach(pmex => {
        console.log(JSON.stringify(pmex, null, 2))
        switch (pmex.item.name) {
            case "Create post": {
                post = pmex.response.json() as NonNullable<typeof post>
                expect(pmex.response.code).toBe(200)
                break
            }
            case "GET posts": {
                expect(post!.content).toBe(environment.values[4]?.value)
                expect(pmex.response.code).toBe(200)
                break
            }
            case "GET posts unauthenticated": {
                expect(pmex.response.code).toBe(401)
                break
            }
            default: {
                expect(pmex.response.code).toBe(200)
                break
            }
        }
    })
})