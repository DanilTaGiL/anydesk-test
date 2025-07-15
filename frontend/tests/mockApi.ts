import { vi } from 'vitest'
import { api } from '../src/utils/api'
import { fakeDetails, fakeUsers } from '../src/utils/testData'


export function mockApi() {
  vi.spyOn(api, 'get').mockImplementation((url: string) => {
    console.log("request", url)
    if (url === '/user/all') return Promise.resolve({ data: fakeUsers })
    if (url.match(/^\/user\/(\d+)/)) {
      const id = +url.slice(-1)
      return Promise.resolve( { data: fakeDetails[id] })
    }
    throw new Error(`unexpected url ${url}`)
  })
}
