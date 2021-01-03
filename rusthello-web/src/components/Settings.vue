<template>
  <div class="row justify-content-center">
    <div class="col-6">
      <form>
        <fieldset>
          <legend>You will be the player</legend>
          <div class="form-check">
            <input
              type="radio"
              v-model="humanPlayer"
              :value="WPlayer.Black"
              id="player_black"
              class="form-check-input"
            />
            <label for="player_black" class="form_check_label">Black</label>
          </div>
          <div class="form-check">
            <input
              type="radio"
              v-model="humanPlayer"
              :value="WPlayer.White"
              id="player_white"
              class="form-check-input"
            />
            <label for="player_white" class="form_check_label">White</label>
          </div>
          <div class="py-3">
            <legend>The computer's level</legend>
            <input
              type="range"
              class="form-range"
              v-model="computerStrength"
              min="4"
              max="9"
              step="1"
            />
          </div>
        </fieldset>
        <button class="btn btn-primary" @click.prevent="letsPlay">
          Let's play !
        </button>
      </form>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";

import { WPlayer } from "rusthello-wasm";

export default defineComponent({
  name: "Settings",
  data: function () {
    return {
      WPlayer, // required for template
      humanPlayer: WPlayer.Black as WPlayer,
      computerStrength: 0 as number,
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
    this.humanPlayer = this.human;
    this.computerStrength = this.level;
  },

  methods: {
    letsPlay: function () {
      this.$emit("letsPlay", {
        human: this.humanPlayer,
        level: this.computerStrength,
      });
    },
  },
});
</script>
