<template>
  <div>
    <counter></counter>
    <stocks></stocks>
  </div>
</template>

<script>
import stocks from './components/stocks'
import counter from './components/counter'

export default {
  components: {
    stocks, counter
  }
}
</script>