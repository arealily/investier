<template>
    <div class="uk-flex-center">
        <h2 class="uk-h2">Stocks</h2>
        <div class="uk-grid" uk-grid>
            <div class="uk-width-1-4" v-for="stock in stocks" v-bind:key="stock.symbol">
                <div class="uk-card uk-card-default uk-card-body">
                    <h3 class="uk-card-title">{{stock.symbol}}</h3>
                    <dl class="uk-description-list">
                        <dt>数量</dt>
                        <dd>{{stock.amount}}</dd>
                    </dl>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import { mapState } from "vuex";

export default {
    name: 'stocks',
    computed: {
      ...mapState(['stocks']),
    },
    mounted () {
       this.$store.dispatch('getCurrentStocks') 
    },
}
</script>