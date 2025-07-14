import { defineStore } from 'pinia'
import { api } from '@/utils/api.ts'
import type { TaskDetails, TaskListItem, TasksState } from '@/utils/types/tasks.ts'

export const useTasksStore = defineStore('tasks', {
  state: (): TasksState => ({
    tasksList: [],
    taskDetails: {},
    editor: {
      open: false,
      selected: null,
      mode: 'edit'
    },
  }),

  actions: {
    async getAllTasks() {
      const res = await api.get('/task/all')
      this.tasksList = res.data as TaskListItem[]
    },
    async getTaskDetails(id: number) {
      this.clearSelection()

      if (!this.taskDetails[id]) {
        const res = await api.get(`/task/${id}`)
        this.taskDetails[id] = res.data as TaskDetails
      }
      this.editor.selected = {...this.taskDetails[id]}
    },
    clearSelection() { this.editor.selected = null },

    async updateTask(details: TaskDetails) {
      await api.put(`/task/${details.id}`, details)
      this.taskDetails[details.id] = details

      // sync title and category with tasks list
      const idx = this.tasksList.findIndex(t => t.id === details.id)
      if (idx !== -1) {
        this.tasksList[idx] = { id: details.id, title: details.title, category: details.category }
      }
      this.editor.open = false
    },

    async createTask(details: TaskDetails) {
      await api.post(`/task`, details)
      this.editor.open = false
      await this.getAllTasks()
    },

    async deleteTask(details: TaskDetails) {
      await api.delete(`/task/${details.id}`)
      this.editor.open = false
      await this.getAllTasks()
    }
  },
})
