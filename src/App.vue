<script setup lang="ts">
// импортинг из vue 2 функции
// onMounted - запускает код после отображения всех компанентов
// ref - быстрая переменная

import {onMounted, ref} from "vue";

import  Database from "@tauri-apps/plugin-sql";

// строем структуру одного сообщения
interface Message{
  id: number;
  author:string,
  body:string,
  created_at:string;
}

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

async function sendMessage() {
  const body = draft.value.trim();

  if (!body) return;

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

      await  loadMessages()

      status.value = "история соо локальна"
    }catch (error){
      console.error(error)

      status.value = "ошибка к подключению bd"
    }

  })


</script>

<template>
  <main class="app">
    <header class="header">
      <div>
        <h1>
          super messenger
        </h1>
        <p> {{status}}</p>
      </div>
      <span class="badge">
        локально
      </span>
    </header>

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

        <article v-for ="message in messages"
        :key = "message.id"
        class = "message">
          <p>
            {{message.body}}
          </p>

          <footer>
            <span>{{message.created_at}}</span>
          </footer>
        </article>
      </div>
      <form class="composer"
      @submit.prevent ="sendMessage()">
        <input
            v-model="draft"
            type="text"
            placeholder="напишите соо"
            autocomplete="off"
        />
        <button type="submit">отправить</button>
      </form>
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
.header{
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 18px 24px;
  border-bottom: blueviolet;
}

.header h1{
  margin: 0;
  font-size: 10px;
}

.header p{
  margin: 4px 0 0;
  font-size: 10px;
  color: darkgreen;
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

.message{
  align-self: flex-end;
  max-width: 70%;
  margin: 0;
  padding: 10px 12px;
  background: saddlebrown;

}

.message p{
  margin: 0;
  line-height: 1.45;
  overflow-wrap: anywhere;
}

.message footer{
  display: flex;
  justify-content: flex-end;
  gap: 5px;
  margin-top: 6px;
  color: lawngreen;
  font-size: 10px;
}

.composer{
  display: flex;
  gap:10px;
  padding: 16px 20px;
  border-top: 1px solid #252830;
  background: #22224e;
}

.composer input{
  flex: 1;
  min-width: 0;
  padding: 12px 14px;
  border: 1px solid #ffffff;
  border-radius: 6px;
  outline: none;
  color: white;
  background: #221313;
  font: inherit;

}

.composer input:focus{
  border-color: lightseagreen;
}

.composer button{
  padding: 0 18px;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  color: white;
  background: lightseagreen;
  font: inherit;
  font-weight: 600;

}

.composer button:hover{
  background: #238a6f;
}
</style>