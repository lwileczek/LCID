<template>
  <div class='blue merida'>
    <div ref='boardDiv' class='cg-board-wrap'></div>
  </div>
</template>

<script setup lang='ts'>
import { ref, onMounted } from 'vue'
import { Chessground } from 'chessground'
import { Api } from 'chessground/api'
import { Key, MoveMetadata } from 'chessground/types'
import { invoke } from '@tauri-apps/api/tauri'

let turn = true
let board: Api
const boardDiv = ref<HTMLElement>()
const getTurnColor = () => {
  return turn ? 'white' : 'black'
}
const changeTurn = (o: Key, d: Key, m: MoveMetadata) => {
  turn = !turn
  console.log(o,d,m)
  console.log('Here is the FEN!', board.getFen())
  invoke('check_legal_moves', { gameFen: board.getFen() })
  board.set({
    turnColor: getTurnColor(),
    movable: {
      color: getTurnColor()
    },
  })
}
onMounted(() => {
  if (boardDiv.value != undefined) {
    board = Chessground(boardDiv.value, {
      movable: {
        color: getTurnColor(),
        showDests: false,
        events: {
          after: (orig,dest,metadata) => {
            changeTurn(orig, dest, metadata)
          }
        }
      },
      premovable: { enabled: false },
      events: {
        move: (orig, dest, capturedPiece) => {
          console.log('A piece moved from ', orig, ' to ', dest)
          if (capturedPiece != undefined) {
            console.log('A piece was captured', capturedPiece)
          }
        },
      },
    })
  }
})
</script>

<style>
</style>
