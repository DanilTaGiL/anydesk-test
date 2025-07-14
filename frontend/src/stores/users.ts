import { defineStore } from 'pinia'
import { api } from '@/utils/api.ts'
import type { UserDetails, UserListItem, UsersState } from '@/utils/types/users.ts'

export const useUsersStore = defineStore('users', {
  state: (): UsersState => ({
    userList: [],
    userDetails: {},
  }),

  actions: {
    async getAllUsers() {
      const res = await api.get('/user/all')
      this.userList = res.data as UserListItem[]
    },

    async getUserDetails(user: UserListItem) {
      if (this.userDetails[user.id] != null) return

      const res = await api.get(`/user/${user.id}`)
      this.userDetails[user.id] = res.data as UserDetails
    },
  },
})
