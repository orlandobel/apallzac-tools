<script setup lang="ts">
// https://raw.githubusercontent.com/mozilla/pdf.js/ba2edeae/web/compressed.tracemonkey-pldi-09.pdf
import { ref, onMounted } from 'vue';
import { getDocument, GlobalWorkerOptions } from 'pdfjs-dist';
import pdfjsWorker from 'pdfjs-dist/build/pdf.worker?url';

GlobalWorkerOptions.workerSrc = pdfjsWorker;

const canvasRef = ref<HTMLCanvasElement | null>(null);
const pdfUrl = ref<string>('https://raw.githubusercontent.com/mozilla/pdf.js/ba2edeae/web/compressed.tracemonkey-pldi-09.pdf');

onMounted(async () => {
  if (!canvasRef.value) return;
  const loadingTask = getDocument(pdfUrl.value);
  const pdf = await loadingTask.promise;
  const page = await pdf.getPage(1);
  const viewport = page.getViewport({ scale: 1.5 });
  const canvas = canvasRef.value;
  const context = canvas.getContext('2d');
  canvas.height = viewport.height;
  canvas.width = viewport.width;
  await page.render({ canvasContext: context!, viewport }).promise;
});
</script>

<template>
  <section class="flex flex-col items-center justify-center overflow-auto w-full h-full">
    <canvas ref="canvasRef" class="border shadow max-w-full h-auto w-auto block" />
  </section>
</template>