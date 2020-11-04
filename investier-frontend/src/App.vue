<template>
  <div>
    <p class="uk-flex uk-flex-center uk-flex-middle" v-if="msg.length > 0">{{msg}}</p>
    <p v-else>
      no text
    </p>
    <form>
      <input class="uk-input" type="text" v-model="msg">
      <button class="uk-button uk-button-default" @click="clear()">clear</button>
    </form>
  </div>
</template>

<script>
export default {
  data () {
    return {
      msg: 'Hello World!'
    }
  },
  methods: {
    clear () {
      this.msg = ''
    }
  }
}
</script>