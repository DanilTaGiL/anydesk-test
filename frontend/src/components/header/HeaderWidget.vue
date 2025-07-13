<script setup lang="ts">

import MainIcon from '@/components/header/MainIcon.vue'
</script>

<template>
  <v-app-bar color="white" flat class="header border-b">
    <MainIcon/>
  </v-app-bar>
</template>

<style scoped>
.header {
  padding-left: 1rem;
  padding-right: 1rem;
}
</style>
