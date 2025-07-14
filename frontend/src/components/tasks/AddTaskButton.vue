<script setup lang="ts">
import { mdiPlus } from '@mdi/js'
import { useTasksStore } from '@/stores/tasks.ts'
import { useUsersStore } from '@/stores/users.ts'

const tasksStore = useTasksStore()
const usersStore = useUsersStore()

const addNewTask = () => {
  tasksStore.editor.mode = 'create'
  tasksStore.editor.open = true
  tasksStore.editor.selected = {
    id: -1,
    title: 'New Task',
    category: 'BUG',
    description: '',
    creatorId: usersStore.userList[0].id,
    assignedTo: usersStore.userList[0].id,
  }
}
</script>

<template>
  <v-btn @click="addNewTask" :icon="mdiPlus" variant="plain" size="small" />
</template>

<style scoped></style>
