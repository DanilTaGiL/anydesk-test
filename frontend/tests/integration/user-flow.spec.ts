import { describe, expect, it } from 'vitest'
import { mockApi } from '../mockApi'
import { mountWithPlugins } from '../utils'

import { fakeDetails, fakeUsers } from '../../src/utils/testData'
import UserList from '../../src/components/users/UserList.vue'
import UserDetails from '../../src/components/users/UserDetails.vue'
import { flushPromises } from '@vue/test-utils'

describe('[INTEGRATION] Users flow', () => {
  it('загружает список и показывает детали', async () => {
    mockApi()
    const wrapper = mountWithPlugins(UserList)

    /* 1. wait for GET /user/all */
    await flushPromises()

    /* 2. Check that we received 2 users */
    const panelTitles = wrapper.findAll('.v-expansion-panel-title')
    expect(panelTitles.length).eq(2)
    expect(wrapper.text()).toContain(fakeUsers[0].fullName)
    expect(wrapper.text()).toContain(fakeUsers[1].fullName)

    /* 3. User details not shown */
    expect(wrapper.findComponent(UserDetails).exists()).toBe(false)

    /* 4. Click on user to request details */
    await panelTitles[1].trigger('click')

    /* 5. User details appear */
    const detailsCmp = wrapper.findAllComponents(UserDetails)
    expect(detailsCmp.length).eq(1)

    /* 6. Check user details props to be sure */
    expect(detailsCmp[0].props('details')).toEqual(fakeDetails[2])
  })
})
