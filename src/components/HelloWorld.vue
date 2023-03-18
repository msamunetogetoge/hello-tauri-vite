<template>
  <v-container>
    <v-row class="text-center">
      <v-col>
        <v-table :hover="true">
          <thead>
            <tr>
              <th class="text-left">
                Name
              </th>
              <th class="text-left">
                Age
              </th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="item in users" :key="item.name" @click="get_user_description(item.name)">
              <td>{{ item.name }}</td>
              <td>{{ item.age }}</td>
            </tr>
          </tbody>
        </v-table>
      </v-col>
    </v-row>
    <v-row class="text-center">
      <v-col>
        <v-btn @click="hello"> {{ button }}</v-btn>
      </v-col>
    </v-row>
    <v-card v-if="descriptionCard">
      <v-card-title>Description</v-card-title>
      <v-card-text>
        <v-table>
          <thead>
            <tr>
              <th class="text-left">
                Name
              </th>
              <th class="text-left">
                Age
              </th>
              <th class="text-left">
                Class
              </th>
            </tr>
          </thead>
          <tbody>
            <td>{{ userDescription.name }}</td>
            <td>{{ userDescription.age }}</td>
            <td>{{ userDescription.class }}</td>
          </tbody>
        </v-table>
      </v-card-text>
      <v-card-actions>
        <v-btn @click="descriptionCard = false">表示終了</v-btn>
      </v-card-actions>
    </v-card>
  </v-container>
</template>

<script lang='ts'>
import { defineComponent } from 'vue'

// Logo
import { invoke } from '@tauri-apps/api/tauri'
import { User, UserDescription } from "../plugins/user"

export default defineComponent({
  name: 'HelloWorld',

  data() {
    return {
      button: "click me;",
      users: Array<User>,
      descriptionCard: false,
      userDescription: {} as UserDescription,

    }
  },
  methods: {
    async hello() {
      const invoke = window.__TAURI__.invoke;
      let hoge = await invoke('my_custom_command')
      this.button = hoge;
    },
    async get_user_description(name: string) {
      const invoke = window.__TAURI__.invoke;
      this.userDescription = await invoke("get_user_description", { name: name }).then(function (v) {

        return v as UserDescription
      }).catch(function (e) {
        console.log(e)
        return {
          name: "",
          age: 0,
          class: "error"
        } as UserDescription
      })
      this.descriptionCard = true;
    }
  },
  async mounted() {
    this.users = await invoke("fetch_user")
  }
})
</script>
