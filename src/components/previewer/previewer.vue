<script setup lang="ts">
// https://raw.githubusercontent.com/mozilla/pdf.js/ba2edeae/web/compressed.tracemonkey-pldi-09.pdf
import { ref, onMounted, watch } from 'vue';
import { getDocument, GlobalWorkerOptions } from 'pdfjs-dist';
import pdfjsWorker from 'pdfjs-dist/build/pdf.worker?url';

GlobalWorkerOptions.workerSrc = pdfjsWorker;

const canvasRef = ref<HTMLCanvasElement | null>(null);
const pdfUrl = ref<string>('https://raw.githubusercontent.com/mozilla/pdf.js/ba2edeae/web/compressed.tracemonkey-pldi-09.pdf');
const zoomLevel = ref<number>(1.5);

const zoomIn = () => {
  	zoomLevel.value *= 1.2;
};

const zoomOut = () => {
  	zoomLevel.value /= 1.2;
};

const save = () => {
  	console.log('TODO :: implement functionality');
};

const print = () => {
  	console.log('TODO :: implement functionality');
};

onMounted(async () => {
  	await renderPdf();
});

watch(zoomLevel, async () => {
  	await renderPdf();
});

const renderPdf = async () => {
	if (!canvasRef.value) return;

	const loadingTask = getDocument(pdfUrl.value);
	const pdf = await loadingTask.promise;
	const page = await pdf.getPage(1);
	const viewport = page.getViewport({ scale: zoomLevel.value });
	const canvas = canvasRef.value;
	const context = canvas.getContext('2d');

	canvas.height = viewport.height;
	canvas.width = viewport.width;
	
	await page.render({ canvasContext: context!, viewport }).promise;
};
</script>


<template>
	<section class="flex flex-col items-center justify-center">
		<div class="w-full bg-gray-700 flex justify-between items-center px-4 py-0 h-[50px] sticky top-0">
			<!-- This is a spacer, if remove all bar will be reorganized -->
			<div></div>

			<div class="flex items-center gap-2">
				<v-btn variant="text" color="surface" icon="mdi-magnify-minus" @click="zoomOut" />
				<span class="text-sm">{{ Math.round(zoomLevel * 100) }}%</span>
				<v-btn variant="text" color="surface" icon="mdi-magnify-plus" @click="zoomIn" />
			</div>

			<div class="flex gap-2 p-4">
				<v-btn variant="text" color="surface" icon="mdi-content-save" @click="save" />
				<v-btn variant="text" color="surface" icon="mdi-printer" @click="print" />
			</div>
		</div>
		
		<canvas ref="canvasRef" class="border shadow max-w-full h-auto w-auto block mt-4" />
	</section>
</template>