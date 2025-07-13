<script setup lang="ts">
import type { UserDetails } from '@/utils/types.ts'

const props = defineProps<{ details?: UserDetails }>()
</script>

<template>
  <div v-if="props.details" class="user-list-item--details">
    <div>
      <p><b>First name:</b></p>
      <p><b>Last name:</b></p>
      <p><b>Headline:</b></p>
    </div>
    <div>
      <p>{{ props.details.firstName }}</p>
      <p>{{ props.details.lastName }}</p>
      <p v-if="props.details.headline">{{ props.details.headline }}</p>
      <p v-else><i class="text-medium-emphasis">Empty</i></p>
    </div>
  </div>
  <!--  TODO: skeleton?-->
  <div v-else>Loading...</div>
</template>

<style scoped>
.user-list-item--details {
  display: flex;
  gap: 2rem;
}
</style>
