import { expect, test } from '@playwright/test'
import { fakeDetails, fakeUsers } from '../../src/utils/testData.js'

test('[E2E] User flow', async ({ page }) => {
  /* Mock backend requests */
  await page.route('**/user/all', (route) => route.fulfill({ json: fakeUsers, status: 200 }))
  await page.route('**/user/00000000-0000-0000-0000-000000000001', (route) =>
    route.fulfill({ json: fakeDetails[1], status: 200 }),
  )

  /* Check that User List it visible */
  await page.goto('/')
  await expect(page.getByText('User List')).toBeVisible()

  /* Check user list content */
  await expect(page.getByText(fakeUsers[0].fullName)).toBeVisible()
  await expect(page.getByText(fakeUsers[1].fullName)).toBeVisible()

  /* Open user details */
  const panel = page.getByRole('button', { name: fakeUsers[0].fullName })
  await panel.click()

  await expect(page.getByText('Empty')).toBeVisible()
})
