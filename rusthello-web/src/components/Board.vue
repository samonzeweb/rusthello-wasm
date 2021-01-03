<template>
  <div class="outer_board">
    <div class="board_upper_x">
      <BoardLetters />
    </div>

    <div class="board_lower_x">
      <BoardLetters />
    </div>

    <div class="board_left_y">
      <BoardNumbers />
    </div>

    <div class="board_right_y">
      <BoardNumbers />
    </div>

    <div class="board">
      <div
        v-for="(piece, i) of pieces"
        :key="i"
        class="cell"
        @mouseover="setOverCell(i)"
        @mouseleave="setOverCell(-1)"
        @click="doMove(i)"
      >
        <div v-if="piece" :class="pieceClass(piece)"></div>
        <div v-if="isOverValidCell(i)" :class="shadowPlayerPieceClass"></div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { PropType } from "vue";
import { WPlayer, WGame } from "rusthello-wasm";
import { defineComponent } from "vue";

import BoardLetters from "./BoardLetters.vue";
import BoardNumbers from "./BoardNumbers.vue";

export default defineComponent({
  name: "Board",
  emits: ["doMove"],
  data: function () {
    return {
      overCell: -1,
    };
  },

  props: {
    game: { type: WGame, required: true },
    currentPlayer: { type: Number as PropType<WPlayer> | null, required: true },
  },

  computed: {
    pieces: function () {
      const pieces = new Array(64);
      for (let y = 0; y < 8; y++) {
        for (let x = 0; x < 8; x++) {
          pieces[x + 8 * y] = this.game.get_piece(x, y);
        }
      }
      return pieces;
    },

    shadowPlayerPieceClass: function (): Array<string> {
      const pieceClass =
        this.currentPlayer == WPlayer.Black ? "black" : "white";
      return [pieceClass, "shadown_piece"];
    },
  },

  methods: {
    isOverValidCell: function (cellIndex: number): boolean {
      return this.overCell == cellIndex && this._isCellValidMove(cellIndex);
    },

    pieceClass: function (piece: WPlayer): string {
      return piece === WPlayer.Black ? "black" : "white";
    },

    setOverCell: function (cellIndex: number) {
      this.overCell = cellIndex;
    },

    doMove: function (cellIndex: number) {
      if (this._isCellValidMove(cellIndex)) {
        const [x, y] = this._getX(cellIndex);
        this.$emit("doMove", { x, y });
      }
    },

    _getX: function (cellIndex: number): [number, number] {
      const y = Math.floor(cellIndex / 8);
      const x = cellIndex - y * 8;
      return [x, y];
    },

    _isCellValidMove: function (cellIndex: number): boolean {
      if (!this.currentPlayer) {
        return false;
      }

      const [x, y] = this._getX(cellIndex);
      return this.game.is_move_valid(this.currentPlayer, x, y);
    },
  },

  components: {
    BoardLetters,
    BoardNumbers,
  },
});
</script>

<style lang="scss">
$cell_size: 4rem; //80px;
$xy_size: 3rem; //60px;

$border_color: lightgrey;
$border_size: 2px;

$piece_size: 2.5rem; //50px;
$piece_shadow: 5px;
$black-color: dimgrey;
$white-color: linen;

.outer_board {
  width: $cell_size * 8 + $xy_size * 2;
  margin: auto;
  display: grid;
  grid-template-columns: $xy_size repeat(8, $cell_size) $xy_size;
  grid-template-rows: $xy_size repeat(8, $cell_size) $xy_size;
  grid-template-areas:
    ". X X X X X X X X ."
    "Y B B B B B B B B y"
    "Y B B B B B B B B y"
    "Y B B B B B B B B y"
    "Y B B B B B B B B y"
    "Y B B B B B B B B y"
    "Y B B B B B B B B y"
    "Y B B B B B B B B y"
    "Y B B B B B B B B y"
    ". x x x x x x x x .";
  background-color: beige;
}

@mixin board_x {
  width: $cell_size * 8;
  height: $xy_size;
  display: grid;
  grid-template-columns: repeat(8, $cell_size);
  grid-template-rows: $xy_size;
}

@mixin board_y {
  width: $xy_size;
  height: $cell_size * 8;
  display: grid;
  grid-template-columns: $xy_size;
  grid-template-rows: repeat(8, $cell_size);
}

.board_upper_x {
  grid-area: X;
  @include board_x();
}

.board_lower_x {
  grid-area: x;
  @include board_x();
}

.board_left_y {
  grid-area: Y;
  @include board_y();
}

.board_right_y {
  grid-area: y;
  @include board_y();
}

.outer_board p {
  margin: 0;
  padding: 0;
  position: relative;
}
.outer_board p span {
  margin: 0;
  padding: 0;
  display: block;
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  font-size: 1.3rem; //25px;
  font-family: Verdana, sans-serif;
}

.board {
  grid-area: B;

  width: $cell_size * 8;
  display: grid;
  grid-template-columns: repeat(8, $cell_size);
  grid-template-rows: repeat(8, $cell_size);
  justify-content: center;
  background-color: rgb(84, 173, 144);
  border-top: $border_size solid $border_color;
  border-left: $border_size solid $border_color;
}

.cell {
  border-bottom: $border_size solid $border_color;
  border-right: $border_size solid $border_color;
}

@mixin piece {
  position: relative;
  top: 0;
  left: 0;
  width: $piece_size;
  height: $piece_size;
  margin: ($cell_size - $piece_size) / 2;
  border-radius: 50%;
  border: 1px solid black;
  box-shadow: $piece_shadow $piece_shadow $piece_shadow rgba(0, 0, 0, 0.5);
  // fun but useless
  // &:hover {
  //   top: -2px;
  //   left: -2px;
  //   box-shadow: $piece_shadow + 2 $piece_shadow + 2 $piece_shadow + 2
  //     rgba(0, 0, 0, 0.6);
  // }
}

.black {
  @include piece;
  background-color: $black_color;
}

.white {
  @include piece;
  background-color: $white_color;
}

.shadown_piece {
  filter: opacity(0.4);
}
</style>

