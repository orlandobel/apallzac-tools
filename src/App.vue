<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import BeltGradePromossionExam from '@/pages/belt-grade-promossion-exam/belt-grade-promossion-exam.vue'
const greetMsg = ref('')
const name = ref('')

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke('greet', { name: name.value })
}
</script>

<template>
  <belt-grade-promossion-exam />
</template>