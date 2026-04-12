<script setup lang="ts">
// https://raw.githubusercontent.com/mozilla/pdf.js/ba2edeae/web/compressed.tracemonkey-pldi-09.pdf
import { computed, onMounted, ref, watch } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { getDocument, GlobalWorkerOptions } from 'pdfjs-dist';
import pdfjsWorker from 'pdfjs-dist/build/pdf.worker?url';

GlobalWorkerOptions.workerSrc = pdfjsWorker;

const canvasRefs = ref<HTMLCanvasElement[]>([]);
const pdf = ref<string>('');
const zoomLevel = ref<number>(1.5);
const totalPages = ref<number>(0);

const btn_enabled = computed(() => totalPages.value > 0);
const zoom_in_enabled = computed(() => totalPages.value > 0 && zoomLevel.value < 3);
const zoom_out_enabled = computed(() => totalPages.value > 0 && zoomLevel.value > 0.5);

/* Toolbar functions */
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
/* End toolbar functions */

const renderPdf = async () => {
	if (!pdf.value) return;
	
	// Decodificar base64 a Uint8Array
	const binaryString = atob(pdf.value);
	const bytes = new Uint8Array(binaryString.length);
	for (let i = 0; i < binaryString.length; i++) {
		bytes[i] = binaryString.charCodeAt(i);
	}

	const loadingTask = getDocument(bytes);
	const pdfDoc = await loadingTask.promise;
	
	totalPages.value = pdfDoc.numPages;
	
	// Render all pages
	for (let pageNum = 1; pageNum <= pdfDoc.numPages; pageNum++) {
		const page = await pdfDoc.getPage(pageNum);
		const viewport = page.getViewport({ scale: zoomLevel.value });
		const canvas = canvasRefs.value[pageNum - 1];
		
		if (!canvas) {
			console.error(`Canvas for page ${pageNum} not found`);
			continue;
		}
		
		const context = canvas.getContext('2d');
		if (!context) {
			console.error(`Context for page ${pageNum} not found`);
			continue;
		}

		canvas.height = viewport.height;
		canvas.width = viewport.width;
		
		await page.render({ canvasContext: context, viewport }).promise;
	}
};

watch([pdf, zoomLevel], async () => {
  	await renderPdf();
});

listen<string>('document-created', (event) => {
	pdf.value = event.payload;
});

onMounted(() => {
	console.log(btn_enabled.value)
	console.log(zoom_in_enabled.value)
	console.log(zoom_out_enabled.value)
	
	if(pdf.value) {
		renderPdf();
	}
})
</script>


<template>
	<section class="flex flex-col items-center justify-center">
		<div class="w-full bg-gray-700 flex justify-between items-center z-10 px-4 py-0 h-[50px] sticky top-0">
			<!-- Page count display -->
			<div class="text-sm text-white">
				{{ totalPages > 0 ? `${totalPages} páginas` : '' }}
			</div>

			<div class="flex items-center gap-2">
				<v-btn variant="text" icon="mdi-magnify-minus" :disabled="!zoom_out_enabled" @click="zoomOut" />
				<span class="text-sm">{{ Math.round(zoomLevel * 100) }}%</span>
				<v-btn variant="text" icon="mdi-magnify-plus" :disabled="!zoom_in_enabled" @click="zoomIn" />
			</div>

			<div class="flex gap-2 p-4">
				<v-btn variant="text" icon="mdi-content-save" :disabled="!btn_enabled" @click="save" />
				<v-btn variant="text" icon="mdi-printer" :disabled="!btn_enabled" @click="print" />
			</div>
		</div>
		
		<div v-if="pdf" class="flex flex-col items-center gap-4 mt-4">
			<div v-for="index in totalPages" :key="index" class="relative">
				<canvas 
					:ref="(el) => { if (el) canvasRefs[index - 1] = el as HTMLCanvasElement }" 
					class="border shadow max-w-full h-auto w-auto block" 
				/>
			</div>
		</div>
	</section>
</template>