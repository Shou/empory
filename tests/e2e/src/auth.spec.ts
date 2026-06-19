
import { test, expect } from '@playwright/test'

test('register page loads', async ({ page }) => {
  await page.goto('/')

  const inputUsername = page.locator("input[name='regUsername']")
  const inputEmail = page.locator("input[name='regEmail']")
  const inputPassword = page.locator("input[name='regPassword']")

  expect(await inputUsername.count()).toBe(1)
  expect(await inputEmail.count()).toBe(1)
  expect(await inputPassword.count()).toBe(1)

  const username = `playwright${Math.floor(Math.random() * 100000)}`
  await inputUsername.fill(username)
  console.log("username:", await inputUsername.inputValue())
  const email = `playwright${Math.floor(Math.random() * 100000)}@play.wright`
  await inputEmail.fill(email)
  console.log("email:", await inputEmail.inputValue())
  const password = "test test test"
  await inputPassword.fill(password)
  console.log("password:", await inputPassword.inputValue())

  const requestPromise = page.waitForRequest(req => {
    return req.url().includes("/auth/register") && req.method() === "POST"
  })
  const responsePromise = page.waitForResponse(res => {
    return res.url().includes("/auth/register")
  })

  const isValid = await page.locator("form").evaluate((form: HTMLFormElement) => form.checkValidity())
  expect(isValid).toBe(true)
  await page.locator("button[type='submit']").click()

  const request = await requestPromise
  const response = await responsePromise

  expect(request.postDataJSON()).toEqual({
    username,
    email,
    password,
  })

  expect(response.ok())
})