<script setup lang="ts">


import type {Message} from "../types/message.ts";
import MessageBubble from "./MessageBubble.vue";

defineProps<{
  messages:Message[];
}>()
</script>

<template>
  <main class="app">


    <section class="chat">
      <div class="chat-info">
        <h2>первый чат</h2>
        <p> ваш 1 лок мессенджер</p>
      </div>
      <div class="messages">
        <div v-if="messages.length === 0"
             class="empty">


          <strong>
            тут пусто
          </strong>

          <span>напишите первое соо</span>

        </div>


      </div>

      <MessageBubble
        v-for="message in messages"
        :key="message.id"
        :message="message"
      />
    </section>
  </main>
</template>

<style scoped>
.messages{
  flex:1;
  overflow-y:auto ;
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 24px
;
}

.empty{
  margin: auto;
  display: flex;
  flex-direction: column;
  gap: 6px;
  text-align: center;
  color: hotpink;
}
</style>
