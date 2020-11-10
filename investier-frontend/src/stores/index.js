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
        const stocks = await axios.get('http://localhost:8081/stocks').then(
          (res) => res.data,
          () => []
       )
       context.commit('resetStocks', stocks)
      }
    },
  })

