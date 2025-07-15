import { mount } from '@vue/test-utils'
import { createVuetify } from 'vuetify'
import { createTestingPinia } from '@pinia/testing'
import { Component } from 'vue'
import { vi } from 'vitest'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import { aliases, mdi } from 'vuetify/iconsets/mdi-svg'

export function mountWithPlugins(component: Component) {
  return mount(component, {
    global: {
      plugins: [
        createVuetify({
          components,
          directives,
          icons: {
            defaultSet: 'mdi',
            aliases,
            sets: {
              mdi,
            },
          },
        }),
        createTestingPinia({
          stubActions: false,
          createSpy: vi.fn,
        }),
      ],
      stubs: { 'router-link': true, 'router-view': true },
    },
  })
}
