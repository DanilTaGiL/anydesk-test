<script setup lang="ts">
import {ref} from "vue";

const props = defineProps<{
  buttonText?: string,
  buttonColor?: string,
  confirmText: string,
  confirmAction: () => void
}>()

const confirm = ref(false)

const triggerConfirm = () => {
  confirm.value = true
}
</script>

<template>
  <template v-if="$slots.trigger">
    <slot name="trigger" :onClick="triggerConfirm"/>
  </template>
  <template v-else>
    <v-btn
        @click="confirm = true"
        variant="plain" :color="buttonColor"
    >
      {{ props.buttonText }}
    </v-btn>
  </template>

  <v-overlay
      v-model="confirm"
      :open-on-click="false"
      location="center center"
      activator="parent"
      scroll-strategy="reposition"
      class="justify-center"
      style="padding-top: 8rem"
  >
    <v-sheet :elevation="2" rounded="lg" class="confirm">
      <div class="confirm__content">
        <h4 v-html="confirmText" />
        <div class="confirm__actions">
          <v-btn
              @click="confirm = false; confirmAction()"
              variant="flat" rounded="lg" color="error"
          >
            Continue
          </v-btn>
          <v-btn
              @click="confirm = false"
              variant="outlined" rounded="lg"
          >
            Cancel
          </v-btn>
        </div>
      </div>
    </v-sheet>
  </v-overlay>
</template>

<style scoped>
.confirm {
  padding: 1rem;
}

.confirm__content {
  gap: 1rem;

  display: flex;
  flex-direction: column;
  align-items: center;
}

.confirm__actions {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}
</style>
