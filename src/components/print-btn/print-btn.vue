<script lang="ts" setup>
import { onMounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { writeFile, remove } from '@tauri-apps/plugin-fs'
import { tempDir, join } from '@tauri-apps/api/path'

interface PrinterInfo {
    name: string
    is_default: boolean
}

const props = defineProps({
    base64: {
        type: String,
        required: true
    },
    disabled: {
        type: Boolean,
        required: false,
        default: false
    }
})

const printers = ref<PrinterInfo[]>([])
const loadingPrinters = ref(false)
const snackbar = ref(false)
const snackbarMessage = ref('')

const showError = (message: string) => {
    snackbarMessage.value = message
    snackbar.value = true
}

const printDocument = async (printer: string) => {
    const tempDirPath = await tempDir()
    const tempFileName = `temp_print_${Date.now()}.pdf`
    const fullTempPath = await join(tempDirPath, tempFileName)

    try {
        const base64Data = props.base64.replace(/^data:application\/pdf;base64,/, '')
        const binaryData = atob(base64Data)
        const bytes = new Uint8Array(binaryData.length)
        for (let i = 0; i < binaryData.length; i++) {
            bytes[i] = binaryData.charCodeAt(i)
        }

        await writeFile(fullTempPath, bytes)
        await invoke('print_pdf_file', { printer, path: fullTempPath })
    } catch (error) {
        console.error('Print failed', error)
        showError(`Error al imprimir: ${error}`)
    } finally {
        try {
            await remove(fullTempPath)
        } catch (cleanupError) {
            console.error('Failed to cleanup temporary file', cleanupError)
        }
    }
}

const VIRTUAL_PRINTERS = [
    'Fax',
    'Microsoft Print to PDF',
    'Save as PDF',
    'OneNote',
    'OneNote (Desktop)',
    'OneNote for Windows 10',
    'cups-pdf',
]

const fetchPrinters = async () => {
    loadingPrinters.value = true
    try {
        const listed: PrinterInfo[] = await invoke('get_printers')
        const availablePrinters = listed.filter(p => !VIRTUAL_PRINTERS.includes(p.name))
        printers.value = availablePrinters

        if (availablePrinters.length === 0) {
            showError('No se detectaron impresoras')
        }
    } catch (error) {
        console.error('Failed to fetch printers', error)
        showError(`No se pudieron obtener las impresoras: ${error}`)
    } finally {
        loadingPrinters.value = false
    }
}

const onMenuOpen = async () => {
    if (printers.value.length === 0) {
        await fetchPrinters()
    }
}

onMounted(fetchPrinters)
</script>

<template>
    <v-menu @update:model-value="opened => opened && onMenuOpen()">
        <template v-slot:activator="{ props }">
            <v-btn variant="text" icon="mdi-printer" v-bind="props" :disabled="disabled" />
        </template>

        <v-list>
            <v-list-item v-if="loadingPrinters" disabled>
                <template v-slot:prepend>
                    <v-progress-circular indeterminate size="18" width="2" class="mr-2" />
                </template>
                Buscando impresoras…
            </v-list-item>

            <v-list-item
                v-else
                v-for="printer in printers"
                :key="printer.name"
                @click="printDocument(printer.name)"
            >
                {{ printer.name }}
            </v-list-item>
        </v-list>
    </v-menu>

    <v-snackbar v-model="snackbar" color="error" :timeout="5000" location="top end">
        {{ snackbarMessage }}
        <template v-slot:actions>
            <v-btn variant="text" @click="snackbar = false">Cerrar</v-btn>
        </template>
    </v-snackbar>
</template>

<style scoped>
:deep(.v-list-item__content) {
    padding: 2px 16px !important;
}

:deep(.v-snackbar__wrapper) {
	margin: 8px !important;
	padding: 8px !important;

	gap: 12px !important;
}
</style>
