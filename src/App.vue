<template>
  <v-app>
    <v-main>
      <Greet />
      <HelloWorld />
    </v-main>
  </v-app>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import HelloWorld from './components/HelloWorld.vue'
import Greet from './components/Greet.vue';

export default defineComponent({
  name: 'App',

  components: {
    HelloWorld,
    Greet,
  },

  data() {
    return {
      //
    }
  },
})
</script>
