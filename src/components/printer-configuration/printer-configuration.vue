<script lang="ts" setup>
import { onMounted, ref, useTemplateRef } from 'vue';
import { getPrinters } from 'tauri-plugin-printer-wkhtml-bin'

const printer_config = useTemplateRef("printer_config")

const printers = ref<any[]>([])
const selectedPrinter = ref<Object | undefined>(undefined)

onMounted(async () => {
    printer_config.value?.showModal()
    printers.value = JSON.parse(await getPrinters())
})
</script>

<template>
    <dialog ref="printer_config"
        class="fixed flex h-[90svh] w-[75svw] bg-surface backdrop:bg-gray-900 backdrop:opacity-90 m-auto p-0 z-50 rounded-lg overflow-hidden">

        <!-- Izquierda: Vista previa -->
        <div class="flex-[2] bg-gray-700 flex flex-col items-center justify-between p-6">
            <h2 class="self-start text-lg font-medium text-white">Imprimir</h2>

            <div class="flex-1 flex items-center justify-center w-full">
                <div class="bg-white shadow-2xl w-44 h-60 rounded-sm flex items-center justify-center text-gray-400 text-sm select-none">
                    <span>Vista previa</span>
                </div>
            </div>

            <!-- Controles de página -->
            <div class="flex items-center gap-3 text-gray-400 text-sm">
                <button class="px-2 py-0.5 rounded hover:bg-white/10 disabled:opacity-30 transition-colors" disabled>&#8592;</button>
                <span>1 / 1</span>
                <button class="px-2 py-0.5 rounded hover:bg-white/10 disabled:opacity-30 transition-colors" disabled>&#8594;</button>
            </div>
        </div>

        <!-- Derecha: Opciones de configuración -->
        <div class="flex-1 min-w-[260px] flex flex-col bg-gray-800 border-l border-white/10 text-gray-200">
            <div class="flex-1 overflow-y-auto px-5 py-5 space-y-6">

                <!-- Destino -->
                <span class="flex items-center gap-3">
                    <label class="flex-1 text-sm text-gray-200">Destino</label>
                    <select class="flex-1 bg-gray-700 border border-white/10 rounded-md max-w-s-35 px-3 py-2 text-sm text-gray-200 focus:outline-none focus:border-primary"
                        v-model="selectedPrinter"
                    >
                        <option 
                            class="bg-gray-700 text-gray-200 hover:bg-gray-600"
                            v-for="(printer, index) in printers" 
                            :key="index" 
                            :value="printer"
                        >
                            {{ printer.Name }}
                        </option>
                    </select>
                </span>

                <!-- Páginas -->
                <span class="flex items-center gap-3">
                    <label class="flex-1 text-sm text-gray-200">Páginas</label>
                    <select class="flex-1 bg-gray-700 border border-white/10 rounded-md px-3 py-2 text-sm text-gray-200 focus:outline-none focus:border-primary">
                        <option value="all">Todas</option>
                        <option value="current">Página actual</option>
                        <option value="custom">Personalizado</option>
                    </select>
                </span>

                <!-- Copias -->
                <span class="flex items-center gap-3">
                    <label class="flex-1 text-sm text-gray-200">Copias</label>
                    <input type="number" value="1" min="1"
                        class="aspect-square bg-white/5 border border-white/10 rounded-md max-h-10 p-2 text-sm text-gray-200 text-center focus:outline-none focus:border-primary" />
                </span>

                <!-- Color -->
                <span class="flex items-center gap-3">
                    <label class="flex-1 text-sm text-gray-200">Color</label>
                    <select class="flex-1 bg-gray-700 border border-white/10 rounded-md px-3 py-2 text-sm text-gray-200 focus:outline-none focus:border-primary">
                        <option value="bw">Blanco y negro</option>
                        <option value="color" selected>Color</option>
                    </select>
                </span>

                <!-- Tamaño de papel -->
                <span class="flex items-center gap-3">
                    <label class="flex-1 text-sm text-gray-200">Tamaño de papel</label>
                    <select class="flex-1 bg-gray-700 border border-white/10 rounded-md px-3 py-2 text-sm text-gray-200 focus:outline-none focus:border-primary">
                        <option value="letter">Carta (Letter)</option>
                        <option value="a4">A4</option>
                        <option value="legal">Legal</option>
                        <option value="a3">A3</option>
                        <option value="b5">B5</option>
                    </select>
                </span>

                <!-- Márgenes -->
                <span class="flex items-center gap-3">
                    <label class="flex-1 text-sm text-gray-200">Márgenes</label>
                    <select class="flex-1 bg-gray-700 border border-white/10 rounded-md px-3 py-2 text-sm text-gray-200 focus:outline-none focus:border-primary">
                        <option value="default">Predeterminados</option>
                        <option value="none">Sin márgenes</option>
                        <option value="minimum">Mínimos</option>
                        <option value="custom">Personalizados</option>
                    </select>
                </span>
            </div>

            <!-- Footer: Acciones -->
            <div class="px-5 py-4 border-t border-white/10 flex justify-end gap-3">
                <button class="px-4 py-2 text-sm text-gray-300 rounded-md hover:bg-white/10 transition-colors">
                    Cancelar
                </button>
                <button class="px-5 py-2 text-sm font-medium bg-primary text-white rounded-md hover:bg-primary/90 transition-colors">
                    Imprimir
                </button>
            </div>
        </div>
    </dialog>
</template>
