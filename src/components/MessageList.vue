<script setup lang="ts">
import type { Message } from "../types/message";
import MessageBubble from "./MessageBubble.vue";

defineProps<{
  messages: Message[];
}>();
</script>

<template>
  <div class="messages">
    <div v-if="messages.length === 0" class="empty">
      <strong>тут пусто</strong>
      <span>напишите первое сообщение</span>
    </div>


    <MessageBubble
        v-for="message in messages"
        :key="message.id"
        :message="message"
    />
  </div>
</template>

<style scoped>
.messages {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 24px;
}

.empty {
  margin: auto;
  display: flex;
  flex-direction: column;
  gap: 6px;
  text-align: center;
  color: hotpink;
}
</style>
