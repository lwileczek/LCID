<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
// import HelloWorld from './components/HelloWorld.vue'
import ChessBoard from './components/ChessBoard.vue'
</script>

<template>
  <div>
    <ChessBoard />
  </div>
</template>

<style>
@import './assets/chessground.css';
/*@import './assets/theme.css';*/
@import './assets/chessboard.brown.css';
@import './assets/embedded-pieces.css';
/*
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
*/
</style>
