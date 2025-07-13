<script setup lang="ts">

import { useUsersStore } from '@/stores/users.ts'
import { onMounted } from 'vue'

const usersStore = useUsersStore()

onMounted(async () => {
  await usersStore.getAllUsers()
})
</script>

<template>
  <div>User List</div>
  <v-expansion-panels variant="accordion">
    <v-expansion-panel
      v-for="user in usersStore.userList"
      :key="user.id"
    >
      <v-expansion-panel-title>
        <div>
          {{user.fullName}}
        </div>
        <v-badge color="success" inline :content="user.role" />
      </v-expansion-panel-title>
      <v-expansion-panel-text>
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
        <v-badge color="success" inline content="2" />
      </v-expansion-panel-text>
    </v-expansion-panel>
  </v-expansion-panels>
</template>

<style scoped></style>
