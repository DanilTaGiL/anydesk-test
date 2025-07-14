<script setup lang="ts">
import { useTasksStore } from '@/stores/tasks.ts'
import { useUsersStore } from '@/stores/users.ts'
import type { UserListItem } from '@/utils/types/users.ts'

const tasksStore = useTasksStore()
const usersStore = useUsersStore()

const itemProps = (item: UserListItem) => {
  return {
    title: item.fullName,
    subtitle: item.role,
  }
}
</script>

<template>
  <v-dialog v-model="tasksStore.editor.open" max-width="500">
    <v-card>
      <template v-slot:text>
        <v-skeleton-loader v-if="!tasksStore.editor.selected" type="article"></v-skeleton-loader>
        <div v-else>
          <v-textarea
            v-model="tasksStore.editor.selected.title"
            label="Title"
            variant="outlined"
            rows="1"
            no-resize
          ></v-textarea>

          <v-textarea
            v-model="tasksStore.editor.selected.description"
            label="Description"
            variant="outlined"
            rows="3"
          ></v-textarea>

          <v-select
            v-model="tasksStore.editor.selected.category"
            :items="['BUG', 'TASK', 'RESEARCH']"
            label="Category"
            variant="outlined"
          ></v-select>

          <v-select
            v-model="tasksStore.editor.selected.creatorId"
            :disabled="tasksStore.editor.mode == 'edit'"
            :item-props="itemProps"
            :items="usersStore.userList"
            :item-value="(item) => item.id"
            label="Creator"
            variant="outlined"
          ></v-select>

          <v-select
            v-model="tasksStore.editor.selected.assignedTo"
            :item-props="itemProps"
            :items="usersStore.userList"
            :item-value="(item) => item.id"
            label="Assigned To"
            variant="outlined"
          ></v-select>
        </div>
      </template>

      <v-divider></v-divider>

      <v-card-actions class="bg-surface-light">
        <v-btn v-if="tasksStore.editor.selected" @click="tasksStore.updateTask(tasksStore.editor.selected)" text="Save"></v-btn>
        <v-spacer></v-spacer>
        <v-btn text="Cancel" variant="plain" @click="tasksStore.editor.open = false"></v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<style scoped></style>
