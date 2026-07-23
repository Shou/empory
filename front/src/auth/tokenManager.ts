
import { QueryClient, queryOptions, useQuery } from '@tanstack/react-query'
import { BASE_URL } from '../config'

export const queryClient = new QueryClient()

const refreshQuery = queryOptions({
    queryKey: ["refresh"],
    staleTime: 5 * 60 * 1000,
    queryFn: async () => {
        console.log("getRefresh")
        const url = BASE_URL + "/auth/refresh"
        const res = await fetch(url, {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
            },
        })

        if (res.ok) {
            const json = await res.json()
            if (json.expires_at < new Date()) {
                // do something
            }
            return json
        }

        return null
    },
})

export async function getToken() {
    const response = await queryClient.ensureQueryData(refreshQuery)
    if (response) return response.token
    return null
}

export function useToken() {
    const query = useQuery(refreshQuery)
    return {
        token: query.data,
        ...query,
    }
}
