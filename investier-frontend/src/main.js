import UIkit from 'uikit'
import Icons from 'uikit/dist/js/uikit-icons'
import 'uikit/dist/css/uikit.css'
import 'uikit/dist/css/uikit.min.css'
UIkit.use(Icons)

import { createApp } from 'vue'
import { createStore } from 'vuex'

const store = createStore({
    state () {
      return {
        count: 1
      }
    }
  })

import App from './App.vue'

createApp(App).mount('#app')
app.use(store)
