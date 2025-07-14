export type TasksState = {
  tasksList: TaskListItem[]
  taskDetails: {
    [id: number]: TaskDetails
  }
  editor: TaskEditor
}

export type TaskCategory = 'BUG' | 'TASK' | 'RESEARCH'

export type TaskListItem = {
  id: number
  title: string
  category: TaskCategory
}

export type TaskDetails = {
  id: number
  title: string
  category: TaskCategory
  creatorId: string
  description?: string
  assignedTo?: string
}

export type TaskEditor = {
  selected: TaskDetails | null,
  open: boolean
  mode: 'edit' | 'create'
}
