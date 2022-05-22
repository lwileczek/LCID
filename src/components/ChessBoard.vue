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

//interface LegalMoves {
//    [index: string]: Key[]
//}

let turn = true
let board: Api
const boardDiv = ref<HTMLElement>()
const getTurnColor = () => {
  return turn ? 'white' : 'black'
}
const changeTurn = (o: Key, d: Key, m: MoveMetadata) => {
  turn = !turn
  console.log(o,d,m)
  invoke('check_legal_moves', { gameFen: board.getFen() })
    .then((legalMoves) => {
      //const typedMoved = {}
      console.log('Got this from the rust', legalMoves)
      //Key is not assignable from String :(((((
      //for (const rustKey in legalMoves) {
      //  const k: Key = rustKey
      //  const moves: Key[] = legalMoves[rustKey]
      //  typedMoved[k] = moves
      //}
      board.set({
        turnColor: getTurnColor(),
        movable: {
          color: getTurnColor(),
        },
      })
    })
  //board.set({
  //  turnColor: getTurnColor(),
  //  movable: {
  //    color: getTurnColor(),
  //  },
  //})
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
