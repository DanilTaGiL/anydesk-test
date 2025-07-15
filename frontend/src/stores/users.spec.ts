import { createPinia, setActivePinia } from 'pinia'
import { useUsersStore } from '@/stores/users'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { api } from '@/utils/api'
import { fakeDetails, fakeUsers } from '@/utils/testData.ts'

vi.mock('@/utils/api', () => ({ api: { get: vi.fn() } }))

describe('[UNIT] users store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  it('getAllUsers correctly save list', async () => {
    ;(api.get as any).mockResolvedValue({ data: fakeUsers })
    const store = useUsersStore()
    expect(store.userList).toHaveLength(0)

    await store.getAllUsers()
    expect(api.get).toHaveBeenCalledWith('/user/all')
    expect(store.userList).toHaveLength(2)
  })

  it('getUserDetails cache response', async () => {
    ;(api.get as any).mockResolvedValue({ data: fakeDetails[1] })
    const store = useUsersStore()

    await store.getUserDetails(fakeUsers[0])
    await store.getUserDetails(fakeUsers[0])
    expect(api.get).toHaveBeenCalledTimes(1)
  })
})
