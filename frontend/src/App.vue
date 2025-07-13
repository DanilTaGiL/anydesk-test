<script setup lang="ts">
import { RouterView } from 'vue-router'
import HeaderWidget from '@/components/header/HeaderWidget.vue'
</script>

<template>
  <v-app>
    <HeaderWidget />
    <v-main>
      <RouterView />
    </v-main>
  </v-app>
</template>

<style scoped>

</style>
