<script setup lang="ts">
import { onMounted } from 'vue'
import { useTasksStore } from '@/stores/tasks.ts'
import BackgroundPanel from '@/components/common/BackgroundPanel.vue'
import type { UserRole } from '@/utils/types/users.ts'
import type { TaskCategory, TaskListItem } from '@/utils/types/tasks.ts'
import TaskEditor from '@/components/tasks/TaskEditor.vue'
import AddTaskButton from '@/components/tasks/AddTaskButton.vue'

const tasksStore = useTasksStore()

const headers = [
  { title: '#', width: '1rem', align: 'end', key: 'id' },
  { title: 'Category', width: '5rem', align: 'start', key: 'category' },
  { title: 'Title', align: 'start', key: 'title' },
]
const categoryColor: Record<TaskCategory, string> = {
  BUG: 'deep-orange-darken-4',
  TASK: 'blue-darken-4',
  RESEARCH: 'green-darken-4',
}

const colorOf = (role: UserRole | string) =>
  (categoryColor as Record<string, string>)[role] ?? 'grey'
const sortBy = [{ key: 'id' }]

const handleClick = (event: any, row: any) => {
  const clickedItem = row.item as TaskListItem
  tasksStore.editor.mode = 'edit'
  tasksStore.getTaskDetails(clickedItem.id)
  tasksStore.editor.open = true
}

onMounted(async () => {
  await tasksStore.getAllTasks()
})
</script>

<template>
  <div class="tasks">
    <div class="tasks--header">
      <h2>Task List</h2>
      <AddTaskButton />
    </div>

    <BackgroundPanel>
      <v-data-table
        @click:row="handleClick"
        :headers="headers as any"
        :items="tasksStore.tasksList"
        :loading="tasksStore.tasksList.length == 0"
        :sort-by="sortBy"
        striped="even"
        density="comfortable"
      >
        <template v-slot:loading>
          <v-skeleton-loader type="table-row-divider@5"></v-skeleton-loader>
        </template>

        <template v-slot:item.category="{ value }">
          <v-chip :text="value" :color="colorOf(value)" variant="outlined" size="small" label />
        </template>
      </v-data-table>
    </BackgroundPanel>
  </div>

  <TaskEditor />
</template>

<style scoped>
.tasks {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;

  width: 30rem;
}

@media (max-width: 960px) {
  .tasks {
    width: 100%;
  }
}

.tasks--header {
  display: flex;
  align-items: center;
  gap: 0.25rem;

  padding-top: 0.5rem;
}
</style>
