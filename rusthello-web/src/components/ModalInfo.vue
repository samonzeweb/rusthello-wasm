<template>
  <transition name="infomodal">
    <div class="modal-mask" v-if="show">
      <div class="modal-dialog">
        <div class="modal-content">
          <div class="modal-header">
            <h5 class="modal-title">{{ title }}</h5>
            <!-- <button type="button" class="btn-close" @click="onOK"></button> -->
          </div>
          <div class="modal-body">
            <p>{{ message }}</p>
          </div>
          <div class="modal-footer">
            <button type="button" class="btn btn-primary" @click="onOK">
              OK
            </button>
          </div>
        </div>
      </div>
    </div>
  </transition>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "ModalInfo",
  data: function () {
    return {
      show: false,
    };
  },

  props: {
    title: {
      type: String,
      required: false,
    },
    message: {
      type: String,
      required: false,
    },
  },

  methods: {
    updateShow: function () {
      this.show = this.title && this.message ? true : false;
    },
    onOK: function () {
      this.show = false;
      this.$emit("close");
    },
  },

  watch: {
    title: function () {
      this.updateShow();
    },
    message: function () {
      this.updateShow();
    },
  },
});
</script>

<style>
.modal-mask {
  position: fixed;
  z-index: 9998;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  display: table;
  transition: opacity 0.3s ease;
}

.infomodal-enter-from,
.infomodal-leave-to {
  opacity: 0;
}

.infomodal-enter-active,
.infomodal-leave-active {
  transition: opacity 0.5s;
}
</style>