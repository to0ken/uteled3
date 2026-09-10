<script setup lang="ts">

defineProps<{
  status: string;
}>();
</script>

<template>
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
</template>

<style scoped>

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


</style>
