export type UsersState = {
  userList: UserListItem[]
  userDetails: {
    [id: string]: UserDetails
  }
}

export type UserRole = 'ADMIN' | 'SUPPORT' | 'DEVELOPER'

export type UserListItem = {
  id: string
  fullName: string
  role: UserRole
}


export type UserDetails = {
  id: string
  firstName: string
  lastName: string
  headline?: string
  role: UserRole
}
