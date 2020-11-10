import { createStore } from 'vuex'

export default createStore({
    state () {
      return {
        count: 1
      }
    },
    mutations: {
        plus(state) {
            state.count += 1
        },
        minus(state) {
            state.count -= 1
        },
    },
  })

