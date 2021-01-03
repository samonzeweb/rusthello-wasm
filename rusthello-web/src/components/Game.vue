<template>
  <ModalInfo
    :title="info.title"
    :message="info.message"
    @close="onModalClose"
  />

  <div class="game_status">
    <h2 v-if="this.currentPlayer === human" class="text-center">
      It's your turn to play.
    </h2>
    <template v-if="this.currentPlayer === computer">
      <h2>It's the computer turn.</h2>
      <p class="lead">It could take some time...</p>
    </template>
    <template v-if="this.game?.game_over()">
      <h2>Game over.</h2>
      <p v-if="this.game?.winner() === this.human" class="lead">
        Congratulations, you won the game.
      </p>
      <p v-if="this.game?.winner() === this.computer" class="lead">
        Sorry, I won the game.
      </p>
      <p v-if="!this.game?.winner()" class="lead">Tie game, no winner.</p>
    </template>
    <p><a href="#" @click.prevent="restartGame">Start a new game</a></p>
  </div>

  <div class="row">
    <div class="col-10">
      <board
        :game="game"
        :currentPlayer="currentPlayer"
        :key="turnCount"
        @doMove="playerMove($event)"
      />
    </div>
    <div class="col-2">
      <h3>Moves</h3>
      <transition-group name="movesHistory_item" tag="ol" class="movesHistory">
        <li
          v-for="(move, i) of movesHistory"
          :key="i"
          :class="classMoveHistory(move.player)"
          :ref="i === movesHistory.length - 1 ? 'last_move' : ''"
        >
          {{ userFriendlyCoordinates(move.x, move.y) }}
        </li>
      </transition-group>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { PropType, nextTick } from "vue";

import { WPlayer, WGame, Coordinates } from "rusthello-wasm";

import Board from "./Board.vue";
import ModalInfo from "./ModalInfo.vue";

interface Move {
  player: WPlayer;
  x: number;
  y: number;
}

export default defineComponent({
  name: "Game",
  emits: ["restart"],
  data: function () {
    return {
      game: null as WGame | null,
      currentPlayer: null as WPlayer | null,
      turnCount: 0,
      movesHistory: [] as Array<Move>,
      info: { title: null as string | null, message: null as string | null },
    };
  },

  props: {
    human: {
      type: Number as PropType<WPlayer>,
      required: true,
    },
    level: { type: Number, required: true },
  },

  created: function () {
    this.game = WGame.new(this.human, this.level);
    this._manageNextTurn();
  },

  computed: {
    computer: function (): WPlayer {
      return this.human === WPlayer.Black ? WPlayer.White : WPlayer.Black;
    },
  },

  methods: {
    userFriendlyCoordinates: function (x: number, y: number): string {
      return `${String.fromCharCode(65 + x)}${y + 1}`;
    },

    classMoveHistory: function (player: WPlayer): string {
      return player === WPlayer.Black ? "move_black" : "move_white";
    },

    playerMove: function ({ x, y }: { x: number; y: number }) {
      this.game?.player_play(x, y);
      this._addHistoryEntry(this.human, x, y);
      this._manageNextTurn();
    },

    onModalClose: function () {
      this._executeNextTurn();
    },

    restartGame: function () {
      this.$emit("restart");
    },

    _computerMove: function () {
      const { x, y } = this.game?.computer_play() as Coordinates;
      this._addHistoryEntry(this.computer, x, y);
      this._manageNextTurn();
    },

    _manageNextTurn: function () {
      this._updateStatus();

      if (this.game?.game_over()) {
        const title = "The game is over.";
        let message;
        if (this.game?.winner() === this.human) {
          message = "You won the game.";
        } else if (this.game?.winner() === this.computer) {
          message = "You loose the game.";
        } else {
          message = "Nobody won the game.";
        }
        this._showModal(title, message);
        return;
      }

      if (this.game?.opponent_is_blocked()) {
        if (this.currentPlayer == this.human) {
          this._showModal(
            "You play again",
            "The computer can't move, it's still your turn."
          );
        } else {
          this._showModal(
            "You can't play",
            "You can't move, it's still the computers's turn."
          );
        }
        return;
      }

      this._executeNextTurn();
    },

    _executeNextTurn: function () {
      if (this.currentPlayer === this.computer) {
        // We need to let Vue re-render some parts before searching the move
        // as it's blocking.
        //
        // this.$nextTick is not enough, and requestAnimationFrame have to be doubled.
        // See https://github.com/vuejs/vue/issues/9200
        requestAnimationFrame(() => {
          requestAnimationFrame(this._computerMove);
        });
      }
    },

    _updateStatus: function () {
      this.currentPlayer = this.game?.player() as WPlayer | null;
      this._rerenderBoard();
    },

    _rerenderBoard: function () {
      // The game isn't "reactive", then we need to force the Board to re-render.
      // Its key attribute (standard Vue) change after each move.
      this.turnCount += 1;
    },

    _addHistoryEntry: function (player: WPlayer, x: number, y: number) {
      this.movesHistory.push({ player, x, y });
      nextTick(() => {
        const lastHistoryMove: HTMLElement = this.$refs
          .last_move as HTMLElement;
        if (lastHistoryMove) {
          lastHistoryMove.scrollIntoView();
        }
      });
    },

    _showModal: function (title: string, message: string) {
      this.info.title = title;
      this.info.message = message;
    },

    _hideModal: function () {
      this.info.title = null;
      this.info.message = null;
    },
  },

  components: {
    Board,
    ModalInfo,
  },
});
</script>

<style>
.game_status {
  min-height: 150px;
  text-align: center;
}
ol.movesHistory {
  height: 35rem;
  overflow-y: scroll;
}

li.movesHistory_item-enter-from {
  opacity: 0;
  font-size: 150%;
}

li.movesHistory_item-enter-active {
  transition: all 1s;
}

.move_black {
  list-style-type: disc;
}

.move_white {
  list-style-type: circle;
}
</style>