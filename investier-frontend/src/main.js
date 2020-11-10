import UIkit from 'uikit'
import Icons from 'uikit/dist/js/uikit-icons'
import 'uikit/dist/css/uikit.css'
import 'uikit/dist/css/uikit.min.css'
UIkit.use(Icons)

import { createApp } from 'vue'
import store from './stores'
import App from './App.vue'

createApp(App).use(store).mount('#app')
