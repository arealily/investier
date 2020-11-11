import { createStore } from 'vuex'
import axios from "axios";

export default createStore({
    state () {
      return {
        stocks: [],
      }
    },
    mutations: {
        resetStocks(state, stocks) {
          state.stocks = stocks
        }
    },
    actions: {
      async getCurrentStocks(context) {
        const backendAPI = process.env.VUE_APP_BACKEND_ENDPOINT
        const stocks = await axios.get(backendAPI + '/stocks').then(
          (res) => res.data,
          () => []
       )
       context.commit('resetStocks', stocks)
      }
    },
  })

