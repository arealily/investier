<template>
  <div>
    <input type="button" v-on:click="onminus" value="-">
    {{ count }}
    <input type="button" v-on:click="onplus" value="+">
  </div>
</template>

<script>
export default {
    name: 'counter',
    computed: {
        count() {
            return this.$store.state.count
        },
    },
    methods: {
        onplus() {
            this.$store.commit('plus')
        },
        onminus() {
            this.$store.commit('minus')
        },
    }
}
</script>