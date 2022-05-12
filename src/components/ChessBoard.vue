<template>
  <div class="app"> 
    <p>{{name}}</p>
    <input type="string" :value="name" @change="changeName">Pick a New Name
    <div class="blue merida">
      <div class="cg-board-wrap" @click="createBoard">using class now to make hting</div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue'
import { Chessground } from 'chessground'
import { Api } from 'chessground/api'

export default defineComponent({
  name: 'ChessBoard',
  components: {},
  props: {
    fakeProperty: {
      type: String,
      required: false,
      default: 'Hello, property'
    },
  },
  setup(){
    const exDiv = ref<HTMLDivElement>()
    console.log('ref Div')
    console.log(exDiv)
    return {
      exDiv,
      func(){console.log(exDiv.value)}
    }
  },
  data() {
    return {
      name: 'Link',
      board: null as null | Api
    }
  },
  methods: {
    changeName(event: Event) {
      const newName = (event.target as HTMLInputElement).value
      this.name = newName
    },
    createBoard(event: Event){
      Chessground((event.target as HTMLElement), {
        fen: 'r1bq1rk1/5ppp/p1np1b2/1p1Np3/4P3/2P5/PPN2PPP/R2QKB1R b KQ - 2 12',
        turnColor: 'black',
        orientation: 'white'
      })
    },
    mounted() {
      this.board = Chessground(this.$el, {
        fen: 'r1bq1rk1/5ppp/p1np1b2/1p1Np3/4P3/2P5/PPN2PPP/R2QKB1R b KQ - 2 12',
        turnColor: 'black',
        orientation: 'white'
      })
    }
  }
})
</script>

<style>
</style>
