<script setup lang="ts">
import {ref} from "vue";

const emit = defineEmits<{
  send:[body:string]
}>()
const draft = ref("");

function submitMessage(){
  const body = draft.value.trim();

  if(!body) return;

  emit("send", body);

  draft.value = "";
}
</script>

<template>
  <form class="composer"
        @submit.prevent ="submitMessage()">
    <input
        v-model="draft"
        type="text"
        placeholder="напишите соо"
        autocomplete="off"
    />
    <button type="submit">отправить</button>
  </form>
</template>

<style scoped>
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
