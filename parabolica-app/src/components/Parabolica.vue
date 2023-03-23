<template>
  <div class="hello">
    <h1>{{ msg }}</h1>
    <div  class="container">
      <div class="item" v-for="(lap, index) in track" :key="index">
        <div class="racer" v-for="(racer, jindex) in lap" :key="jindex">
          <span>{{racer}}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { mapGetters, mapActions } from "vuex";

export default {
  name: "parabolica-track",
  props: {
    msg: String
  },
  created() {
    setInterval(() => {
      this.$store.dispatch("parabolica/fetchTrack");
    }, 2000);
  },
  computed: {
    ...mapGetters("parabolica", ["track"]),
  },
  methods: {
    ...mapActions("parabolica", ["fetchTrack"]),
  },
}
</script>

<style scoped>
  .container {
    display: flex;
    flex-direction: row;
  }
  .racer {
    padding: 5px;
    border: 1px solid lightgrey;
  }
</style>
