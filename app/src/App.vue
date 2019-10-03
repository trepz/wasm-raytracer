<template>
  <div id="app">
    <div class="canvas-container">
      <canvas width="800" height="600" ref="canvas"></canvas>
    </div>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class App extends Vue {
  async mounted() {
    const wasm = await import("wasm-raytracer");

    const c = <HTMLCanvasElement>this.$refs["canvas"];
    const ctx = c.getContext("2d");

    wasm.render(ctx, 800, 600);
  }
}
</script>

<style lang="less">
#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
