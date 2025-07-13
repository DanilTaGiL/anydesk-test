<script setup lang="ts">

import { useDisplay } from 'vuetify/framework'

const { smAndDown } = useDisplay()
</script>

<template>
  <v-sheet :elevation="2" width="100%" :rounded="smAndDown ? 'false' : 'lg'">
    <slot></slot>
  </v-sheet>
</template>

<style scoped>

</style>
