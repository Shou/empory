
import '@empory/config'
import { defineConfig } from '@playwright/test'

export default defineConfig({
  testDir: './src',

  use: {
    baseURL: `http://${process.env.FRONT_HOST}:${process.env.FRONT_PORT}`,
    headless: true,
  },
})