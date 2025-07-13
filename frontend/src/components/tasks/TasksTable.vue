<script setup lang="ts">
import { onMounted } from 'vue'
import { useTasksStore } from '@/stores/tasks.ts'
import BackgroundPanel from '@/components/common/BackgroundPanel.vue'
import type { UserRole } from '@/utils/types/users.ts'
import type { TaskCategory } from '@/utils/types/tasks.ts'

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

onMounted(async () => {
  await tasksStore.getAllTasks()
})
</script>

<template>
  <div class="tasks">
    <h2 class="tasks--header">Task List</h2>

    <BackgroundPanel>
      <v-data-table
        :headers="headers as any"
        :items="tasksStore.tasksList"
        :loading="tasksStore.tasksList.length == 0"
        hide-default-footer
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
  padding-top: 0.5rem;
}
</style>
