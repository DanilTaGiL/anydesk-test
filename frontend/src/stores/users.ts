import { defineStore } from 'pinia'
import type { UserDetails, UserListItem, UsersState } from '@/utils/types.ts'
import { api } from '@/utils/api.ts'

export const useUsersStore = defineStore('users', {
  state: (): UsersState => ({
    userList: [],
    userDetails: {}
  }),

  actions: {
    async getAllUsers() {
      const res = await api.get('/user/all')
      this.userList = res.data as UserListItem[]
    },

    async getUserDetails(user: UserListItem) {
      const res = await api.get(`/user/${user.id}`)
      this.userDetails[user.id] = res.data as UserDetails
    }
  }
})
