<template>
  <q-layout view="hHh lpR fFf">
    <div class="row no-wrap">
      <div class="col">
        <q-tabs align="left" v-model="tab" class="text-right">
          <q-tab name="Connect" label="Connect" />
          <q-tab name="PageTwo" label="PageTwo" />
        </q-tabs>
      </div>
      <div class="col-auto">
        <q-btn flat round dense :icon="theme" @click="toggleTheme" />
      </div>
    </div>

    <q-page-container>
      <KeepAlive>
        <component :is="tabs[tab]"></component>
      </KeepAlive>
    </q-page-container>
  </q-layout>
</template>

<script setup>
import { ref } from 'vue'
import Connect from './Connect.vue'
import PageTwo from './PageTwo.vue'

import { useQuasar } from 'quasar'

const $q = useQuasar()

const tab = ref('Connect')
const tabs = {
  Connect: Connect,
  PageTwo: PageTwo,
}

const theme = ref('light_mode')

const toggleTheme = () => {
  theme.value = theme.value === 'light_mode' ? 'dark_mode' : 'light_mode'

  $q.dark.toggle()
}
</script>
