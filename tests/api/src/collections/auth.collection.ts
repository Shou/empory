
// NOTE do NOT use postman-collection constructors they HATE you (they are REALLY buggy, poor URL parsing logic etc)
import { type CollectionDefinition } from 'postman-collection'

export const collection: CollectionDefinition = {
    info: {
        name: "Back API",
    },
    item: [
        {
            name: "Authentication",
            item: [
                {
                    name: "Register user",
                    request: {
                        method: "POST",
                        url: "{{baseUrl}}/auth/register",
                        header: [{ key: "Content-Type", value: "application/json" }],
                        body: {
                            mode: "raw",
                            raw: JSON.stringify({
                                email: "{{$randomEmail}}",
                                username: "{{$randomUserName}}",
                                password: "test test test",
                            }),
                        },
                    },
                },
            ],
        },
    ],
}
