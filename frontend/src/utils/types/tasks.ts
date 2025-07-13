export type TasksState = {
  tasksList: TaskListItem[]
  taskDetails: {
    [id: number]: TaskDetails
  }
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
  description: string
  creatorId: string
  assignedTo: string
}
