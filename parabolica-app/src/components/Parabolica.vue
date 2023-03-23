<template>
  <div class="hello">
    <h1>{{ msg }}</h1>

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
    this.$store.dispatch("parabolica/fetchTrack");
  },
  computed: {
    ...mapGetters("parabolica", ["track"]),
  },
  methods: {
    ...mapActions("parabolica", ["fetchTrack"]),
  },
}
</script>

