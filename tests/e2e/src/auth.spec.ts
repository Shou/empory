
import { test, expect } from '@playwright/test'

const username = `playwright${Math.floor(Math.random() * 100000)}`
const email = `playwright${Math.floor(Math.random() * 100000)}@play.wright`
const password = "test test test"

test.describe.configure({ mode: "serial" })

test('user registration', async ({ page }) => {
  await page.goto('/')

  const registerSwitch = page.locator("button[name='register']")
  await registerSwitch.click()

  const inputUsername = page.locator("input[name='regUsername']")
  const inputEmail = page.locator("input[name='regEmail']")
  const inputPassword = page.locator("input[name='regPassword']")

  expect(await inputUsername.count()).toBe(1)
  expect(await inputEmail.count()).toBe(1)
  expect(await inputPassword.count()).toBe(1)

  await inputUsername.fill(username)
  console.log("username:", await inputUsername.inputValue())
  await inputEmail.fill(email)
  console.log("email:", await inputEmail.inputValue())
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

test('user login', async ({ page }) => {
  await page.goto('/')

  const registerSwitch = page.locator("button[name='login']")
  await registerSwitch.click()

  const inputUsername = page.locator("input[name='loginUsername']")
  const inputPassword = page.locator("input[name='loginPassword']")

  expect(await inputUsername.count()).toBe(1)
  expect(await inputPassword.count()).toBe(1)

  await inputUsername.fill(username)
  console.log("username:", await inputUsername.inputValue())
  await inputPassword.fill(password)
  console.log("password:", await inputPassword.inputValue())

  const requestPromise = page.waitForRequest(req => {
    return req.url().includes("/auth/login") && req.method() === "POST"
  })
  const responsePromise = page.waitForResponse(res => {
    return res.url().includes("/auth/login")
  })

  const isValid = await page.locator("form").evaluate((form: HTMLFormElement) => form.checkValidity())
  expect(isValid).toBe(true)
  await page.locator("button[type='submit']").click()

  const request = await requestPromise
  const response = await responsePromise

  expect(request.postDataJSON()).toEqual({
    username,
    password,
  })

  expect(response.ok())
  const json = await response.json()
  expect("token" in json)
})
