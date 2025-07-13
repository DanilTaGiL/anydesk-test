export type UsersState = {
  userList: UserListItem[]
}

export type UserListItem = {
  id: string
  fullName: string
  role: UserRole
}

export type UserRole = 'ADMIN' | 'SUPPORT' | 'DEVELOPER'
