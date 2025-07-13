import { defineStore } from 'pinia'
import type { UserListItem, UsersState } from '@/utils/types.ts'
import { api } from '@/utils/api.ts'

export const useUsersStore = defineStore('users', {
  state: (): UsersState => ({
    userList: [],
  }),

  actions: {
    async getAllUsers() {
      const res = await api.get('/user/all')
      this.userList = res.data as UserListItem[]
    }
  }
})
