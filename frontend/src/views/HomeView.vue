<script setup lang="ts">
import UserList from '@/components/users/UserList.vue'
import TaskList from '@/components/tasks/TaskList.vue'
</script>

<template>
  <div class="home-view">
    <UserList/>
    <TaskList/>
  </div>
</template>

<style scoped>
.home-view {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding-top: 0.5rem;
}
</style>
