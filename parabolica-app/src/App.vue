<template>
  <img alt="parabolica logo" src="./assets/logo.png">
  <Parabolica msg="Parabolica Track"/>
</template>

<script>
import Parabolica from './components/Parabolica.vue'

export default {
  name: 'App',
  components: {
    Parabolica,
  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
