<script setup lang="ts">
const listen = () => {
  fetch("event://global").then(r => {
    return r.json();
  }, e => {
    console.error(e);
  }).then(r => console.log(r));
}
</script>

<template>
  <div>
    <button type="button" @click="listen">Beware</button>
  </div>
</template>
