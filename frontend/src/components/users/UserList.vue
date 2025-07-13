<script setup lang="ts">
import { useUsersStore } from '@/stores/users.ts'
import { onMounted } from 'vue'
import UserDetails from '@/components/users/UserDetails.vue'
import type { UserListItem } from '@/utils/types.ts'

const usersStore = useUsersStore()

const openDetailsHandler = async (user: UserListItem, isOpen: boolean) => {
  if (!isOpen) return

  if (usersStore.userDetails[user.id] == null) {
    await usersStore.getUserDetails(user)
  }
}

onMounted(async () => {
  await usersStore.getAllUsers()
})
</script>

<template>
  <div class="user-list">
    <h2>User List</h2>
    <v-expansion-panels variant="accordion">
      <v-expansion-panel
        v-for="user in usersStore.userList"
        :key="user.id"
        @group:selected="(val) => openDetailsHandler(user, val.value)"
      >
        <v-expansion-panel-title>
          <div class="user-list-item--title">
            <p><b>{{ user.fullName }}</b></p>
            <v-badge color="warning" inline :content="user.role" />
          </div>
        </v-expansion-panel-title>

        <v-expansion-panel-text>
          <UserDetails :details="usersStore.userDetails[user.id]" />
        </v-expansion-panel-text>
      </v-expansion-panel>
    </v-expansion-panels>
  </div>
</template>

<style scoped>
.user-list {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;

  width: 30rem;
}

@media (max-width: 960px) {
  .user-list {
    width: 100%;
  }
}

.user-list-item--title {
  display: flex;
  width: 100%;
  justify-content: space-between;
  align-items: center;
}
</style>
