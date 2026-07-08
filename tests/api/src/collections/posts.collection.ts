
// NOTE do NOT use postman-collection constructors they HATE you (they are REALLY buggy, poor URL parsing logic etc)
import { type CollectionDefinition } from 'postman-collection'

export const collection: CollectionDefinition = {
    info: {
        name: "Back API /posts",
    },
    item: [
        {
            name: "Posts",
            item: [
                {
                    name: "Create post",
                    request: {
                        method: "POST",
                        url: "{{baseUrl}}/posts",
                        header: [
                            { key: "Content-Type", value: "application/json" },
                            { key: "Authorization", value: "Bearer {{accessToken}}" },
                        ],
                        body: {
                            mode: "raw",
                            raw: JSON.stringify({
                                content: "{{post}}",
                            }),
                        },
                    },
                },
                {
                    name: "GET posts",
                    request: {
                        method: "GET",
                        url: "{{baseUrl}}/posts",
                        header: [
                            { key: "Content-Type", value: "application/json" },
                            { key: "Authorization", value: "Bearer {{accessToken}}" },
                        ],
                    },
                },
                {
                    name: "GET posts unauthenticated",
                    request: {
                        method: "GET",
                        url: "{{baseUrl}}/posts",
                        header: [
                            { key: "Content-Type", value: "application/json" },
                        ],
                    },
                },
            ],
        },
    ],
}
