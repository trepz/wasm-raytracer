<template>
  <div id="app">
    <h2>wasm raytracer</h2>
    <div class="canvas-container">
      <canvas width="300" height="200" ref="canvas"></canvas>
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
    const imgData = ctx.createImageData(300, 200);
    let i;
    for (i = 0; i < imgData.data.length; i += 4) {
      imgData.data[i+0] = 255;
      imgData.data[i+1] = 0;
      imgData.data[i+2] = 0;
      imgData.data[i+3] = 255 - (i / 800);
    }
    ctx.putImageData(imgData, 0, 0);
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
