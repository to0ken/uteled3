<script setup lang="ts">
// импортинг из vue 2 функции
// onMounted - запускает код после отображения всех компанентов
// ref - быстрая переменная

import {onMounted, ref} from "vue";


import  Database from "@tauri-apps/plugin-sql";

import AppHader from "./components/AppHader.vue";

import MessageList from "./components/MessageList.vue";

import MessageComposer from "./components/MessageComposer.vue";

import type {Message} from "./types/message";

// строем структуру одного сообщения


const draft = ref("");

const messages = ref<Message[]>([])

const status = ref ("подключение ")

// подключеие к бд пока его нет используем
let db: Database | null = null;


// асинк функ загрузки сообщения из бд
async function loadMessages(){
  if(!db) return;

  messages.value = await db.select<Message[]>(
      "SELECT id, author, body, created_at FROM messages ORDER BY id ASC ",
  );
}

async function sendMessage(body:string) {
  if (!db) return;

  // добавление нового соо в бд

  await db.execute(
      "INSERT INTO messages (author, body) VALUES ($1, $2)",
      ["вы", body]
  );

  draft.value = "";
  await loadMessages();
}

  onMounted(async() =>{
    try {
      db = await Database.load("sqlite:messenger.db")

      await loadMessages()

      status.value = "история соо локальна"
    }catch (error){
      console.error(error)

      status.value = "ошибка к подключению bd"
    }

  })


</script>

<template>
  <main class="app">

    <AppHader :status="status"/>


    <section class="chat">
      <div class="chat-info">
        <h2>первый чат</h2>
        <p> ваш 1 лок мессенджер</p>
      </div>

      <MessageList :messages="messages"/>

      <MessageComposer @send="sendMessage"/>
      <div class="messages">
        <div v-if="messages.length === 0"
        class="empty">


          <strong>
            тут пусто
          </strong>

          <span>напишите первое соо</span>

        </div>


      </div>
      <MessageComposer
          @submit.prevent ="sendMessage"
      />
    </section>
  </main>
</template>

<style scoped>
:global(*){
  box-sizing: border-box;
}

:global(html){
   background: aliceblue;
   color-scheme: dark;
 }

:global(body){
  margin: 0;

  font-family:
  Inter,
  system-ui
  -apple-system,
  BlickMacSystemFont,
  "Segoe UI",
  sans-serif;

  color: black;
  background: aqua;
}

.app{
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}


.badge{
  padding: 6px 12px;
  border: 1px solid #343842;
  border-radius: 6px;
  color: crimson;
  background: darkblue;
  font-size: 12px;

}

.chat{
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
}

.chat-info{
  padding: 20px 25px;
  border-bottom: lightseagreen;

}

.chat-info h2{
  margin: 0;
  font-size: 16px;
}

.chat-info p{
  margin: 5px 0 0;
  color: orangered;
  font-size: 13px;
}

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
