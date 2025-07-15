import type { UserDetails, UserListItem } from '@/utils/types/users.ts'

export const fakeUsers = [
  { id: '00000000-0000-0000-0000-000000000001', fullName: 'Danil Suiagin', role: 'ADMIN' },
  { id: '00000000-0000-0000-0000-000000000002', fullName: 'Alice Liddell', role: 'DEVELOPER' },
] as UserListItem[]

export const fakeDetails = {
  1: {
    id: '00000000-0000-0000-0000-000000000001',
    firstName: 'Danil',
    lastName: 'Suiagin',
    headline: undefined,
    role: 'ADMIN',
  },
  2: {
    id: '00000000-0000-0000-0000-000000000002',
    firstName: 'Alice',
    lastName: 'Liddell',
    headline: 'QA engineer',
    role: 'DEVELOPER',
  },
} as {
  [id: number]: UserDetails
}

