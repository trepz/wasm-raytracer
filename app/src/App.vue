<template>
  <div id="app">
    <div class="canvas-container">
      <canvas width="800" height="600" ref="canvas"></canvas>
    </div>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";

@Component
export default class App extends Vue {
  async mounted() {
    const { Canvas } = await import("wasm-raytracer");

    const c = this.$refs["canvas"] as HTMLCanvasElement;
    const ctx = c.getContext("2d");

    const canvas = Canvas.new(800, 600);
    canvas.render(ctx);
  }
}
</script>

<style lang="less">
#app {
  font-family: "Avenir", Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
