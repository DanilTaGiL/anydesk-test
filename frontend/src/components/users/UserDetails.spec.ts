import { describe, expect, it } from 'vitest'

import { mount } from '@vue/test-utils'
import UserDetails from '@/components/users/UserDetails.vue'
import { fakeDetails } from '@/utils/testData.ts'

const skeletonStub = { template: '<div class="fake-skeleton"></div>' }

describe('[COMPONENT] UserDetails', () => {
  it('Renders skeleton when no props', () => {
    const wrapper = mount(UserDetails, {
      global: { stubs: { 'v-skeleton-loader': skeletonStub } },
    })

    expect(wrapper.find('.fake-skeleton').exists()).toBe(true)
    expect(wrapper.find('.user-list-item--details').exists()).toBe(false)
  })

  it('Shows user data when props provided', () => {
    const wrapper = mount(UserDetails, {
      props: { details: fakeDetails[2] },
      global: { stubs: { 'v-skeleton-loader': skeletonStub } },
    })

    expect(wrapper.find('.fake-skeleton').exists()).toBe(false)

    const detailsBlock = wrapper.find('.user-list-item--details')
    expect(detailsBlock.exists()).toBe(true)

    expect(detailsBlock.text()).toContain(fakeDetails[2].firstName)
    expect(detailsBlock.text()).toContain(fakeDetails[2].lastName)
    expect(detailsBlock.text()).toContain(fakeDetails[2].headline)
  })

  it('Show `Empty` when null headline', () => {
    const wrapper = mount(UserDetails, {
      props: { details: fakeDetails[1] },
      global: { stubs: { 'v-skeleton-loader': skeletonStub } },
    })

    const detailsBlock = wrapper.find('.user-list-item--details')
    expect(detailsBlock.text()).toContain('Empty')
  })
})
