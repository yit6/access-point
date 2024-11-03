import { mount } from 'svelte'
import Login from './Login.svelte'

const login = mount(Login, {
  target: document.getElementById('login')!,
})

export default login
