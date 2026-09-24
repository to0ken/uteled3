<script setup lang="ts">
import { ref } from "vue";

const emit = defineEmits<{
  send: [body: string];
}>();

const draft = ref("");

const emojis = ["😀", "😂", "❤️", "👍", "🔥", "😎", "🎉", "🤔"];

function submitMessage() {
  const body = draft.value.trim();
  if (!body) return;
  emit("send", body);
  draft.value = "";
}

function addEmoji(emoji: string) {
  draft.value += emoji;
}
</script>

<template>
  <div class="emoji-panel">
    <button
        v-for="(emoji, index) in emojis"
        :key="index"
        type="button"
        class="emoji-btn"
        @click="addEmoji(emoji)"
        :title="'Добавить ' + emoji"
    >
      {{ emoji }}
    </button>
  </div>


  <form class="composer" @submit.prevent="submitMessage">
    <input
        v-model="draft"
        type="text"
        placeholder="напишите сообщение"
        autocomplete="off"
    />
    <button type="submit">отправить</button>
  </form>
</template>

<style scoped>
.emoji-panel {
  display: flex;
  gap: 8px;
  padding: 8px 16px;
  background: #f5f5f5;
  overflow-x: auto;
  border-top: 1px solid #eee;
}

.emoji-btn {
  background: none;
  border: none;
  font-size: 24px;
  cursor: pointer;
  padding: 4px 8px;
  border-radius: 8px;
  transition: background 0.2s;
}

.emoji-btn:hover {
  background: #e0e0e0;
}

.composer {
  display: flex;
  gap: 8px;
  padding: 16px;
  border-top: 1px solid #ccc;
  background: #fff;
}

.composer input {
  flex: 1;
  padding: 10px 16px;
  border: 1px solid #ccc;
  border-radius: 20px;
  outline: none;
  font-size: 16px;
}

.composer input:focus {
  border-color: #007bff;
}

.composer button[type="submit"] {
  padding: 10px 20px;
  border: none;
  border-radius: 20px;
  background: #007bff;
  color: white;
  font-size: 16px;
  cursor: pointer;
  transition: background 0.2s;
}

.composer button[type="submit"]:hover {
  background: #0056b3;
}
</style>
