import { defineStore } from 'pinia'
import { api } from '@/utils/api.ts'
import type { TaskListItem, TasksState } from '@/utils/types/tasks.ts'

export const useTasksStore = defineStore('tasks', {
  state: (): TasksState => ({
    tasksList: [],
    taskDetails: {},
  }),

  actions: {
    async getAllTasks() {
      const res = await api.get('/task/all')
      this.tasksList = res.data as TaskListItem[]
    },
  }
})
