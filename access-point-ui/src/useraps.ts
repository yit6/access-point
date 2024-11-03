import { mount } from 'svelte'
import UserAPS from './UserAPS.svelte'

const user_aps = mount(UserAPS, {
  target: document.getElementById('user-aps')!,
})

export default user_aps
