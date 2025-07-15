import { createPinia, setActivePinia } from 'pinia'
import { useUsersStore } from '@/stores/users'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { api } from '@/utils/api'
import { _userDetails, _userListItem } from '@/utils/testData.ts'

vi.mock('@/utils/api', () => ({ api: { get: vi.fn() } }))

describe('[UNIT] users store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  it('getAllUsers correctly save list', async () => {
    const data = [_userListItem]
    ;(api.get as any).mockResolvedValue({ data: data })
    const store = useUsersStore()
    expect(store.userList).toHaveLength(0)

    await store.getAllUsers()
    expect(api.get).toHaveBeenCalledWith('/user/all')
    expect(store.userList).toHaveLength(1)
  })

  it('getUserDetails cache response', async () => {
    ;(api.get as any).mockResolvedValue({ data: _userDetails })
    const store = useUsersStore()

    await store.getUserDetails(_userListItem)
    await store.getUserDetails(_userListItem)
    expect(api.get).toHaveBeenCalledTimes(1)
  })
})
