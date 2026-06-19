
import '@birdshit/config'
import { it, expect } from 'vitest'
import newman from 'newman'
import { collection as authCollection } from './collections/auth.collection'

const BASE_URL = `http://${process.env.BACK_HOST}:${process.env.BACK_PORT}`

const environment: newman.NewmanRunOptions["environment"] = {
    values: [
        { key: "baseUrl", value: BASE_URL },
    ],
}

it("postman", async () => {
    console.log("newman env")
    console.log(JSON.stringify(environment, null, 4))
    const summary: newman.NewmanRunSummary = await new Promise((resolve, reject) => {
        console.log(JSON.stringify(authCollection, null, 4))
        newman.run(
            {
                collection: authCollection,
                environment,
                reporters: ["cli"],
            },
            (err, summary1) => {
                if (err) reject(err)
                console.log(JSON.stringify(summary1.run, null, 2))
                resolve(summary1)
            }
        )
    })

    expect(summary.run.failures).to.toHaveLength(0)
    summary.run.executions.forEach(pmex => {
        expect(pmex.response.code).toBe(200)
    })
})