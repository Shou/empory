import dotenv from 'dotenv'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const envName = process.env.APP_ENV ?? "dev"
const envPath = fileURLToPath(
    new URL(`../../.env.${envName}`, import.meta.url)
)
const result = dotenv.config({
    path: envPath,
})
console.log("@birdshit/config loaded:", envPath)