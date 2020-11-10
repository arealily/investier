import UIkit from 'uikit'
import Icons from 'uikit/dist/js/uikit-icons'
import 'uikit/dist/css/uikit.css'
import 'uikit/dist/css/uikit.min.css'
UIkit.use(Icons)

import { createApp } from 'vue'
import axios from 'axios'
import VueAxios from 'vue-axios'

import store from './stores'
import App from './App.vue'

createApp(App).use(VueAxios, axios).use(store).mount('#app')
