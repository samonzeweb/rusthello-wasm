<template>
  <main class="container">
    <div class="text-center py-3">
      <h1>Rusthello WASM</h1>
      <p class="lead">
        This is how a small learning Rust project ended in a web game using
        WebAssembly.
      </p>
      <hr />
    </div>

    <Settings
      v-if="currentPageState === PageState.Settings"
      :human="human"
      :level="level"
      @letsPlay="letsPlay"
    />

    <Game
      v-if="currentPageState === PageState.Game"
      :human="human"
      :level="level"
      @restart="restart"
    />

    <hr />
  </main>
</template>

<script lang="ts">
import { defineComponent } from "vue";

import { WPlayer } from "rusthello-wasm";

import Settings from "./components/Settings.vue";
import Game from "./components/Game.vue";

enum PageState {
  Settings = 1,
  Game = 2,
}

export default defineComponent({
  name: "App",
  data: function () {
    return {
      WPlayer, // required for template
      PageState, // required for template
      currentPageState: PageState.Settings,
      human: WPlayer.Black,
      level: 4,
    };
  },

  methods: {
    letsPlay: function ({ human, level }: { human: WPlayer; level: number }) {
      this.human = human;
      this.level = level;
      this.currentPageState = PageState.Game;
    },
    restart: function () {
      this.currentPageState = PageState.Settings;
    },
  },

  components: {
    Settings,
    Game,
  },
});
</script>
