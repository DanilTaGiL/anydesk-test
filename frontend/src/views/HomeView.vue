<script setup lang="ts">
import UserList from '@/components/users/UserList.vue'
import TasksTable from '@/components/tasks/TasksTable.vue'
</script>

<template>
  <div class="home-view">
    <UserList/>
    <TasksTable/>
  </div>
</template>

<style scoped>
.home-view {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding-top: 0.5rem;

  gap: 1rem;
}
</style>
