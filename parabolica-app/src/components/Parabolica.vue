<template>
  <div class="hello">
    <h1>{{ msg }}</h1>
    <h3>Racers:</h3>
    <h4 v-for="(racer, i) in racer_positions" :orderBy="racer" :key="i">{{i+1}}: {{racer}}</h4>
    <div class="outer">
      <div class="container">
        <div class="item" v-for="(lap, index) in track" :key="index">
          <div 
          v-for="(racer, jindex) in lap" 
          class="racer"
          :class="[
          (racer === 'FireShell') ? 'shell' : 'racer',
          (racer === 'Accelerate') ? 'accelerate' : 'racer'
          ]"
          :key="jindex"
          >
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { mapGetters } from "vuex";

export default {
  name: "parabolica-track",
  props: {
    msg: String
  },
  created() {
    setInterval(() => {
      this.$store.dispatch("parabolica/fetchTrack");
      this.$store.dispatch("parabolica/fetchLap");
      this.$store.dispatch("parabolica/fetchRacerPositions");
    }, 2000);
  },
  computed: {
    ...mapGetters("parabolica", ["track", "lap_number", "racer_positions"]),

  },
}
</script>

<style scoped>
  .outer {
    margin: auto;
    width: 75%;
  }
  .container {
    display: flex;
    flex-direction: row;
  }
  .racer {
    padding: 7px;
    border: 1px solid lightgrey;
  }

  .shell {
    background: darkgreen;
  }

  .accelerate {
    background: lightgreen;
  }
</style>
