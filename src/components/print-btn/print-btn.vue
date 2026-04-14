<script lang="ts" setup>
import { onMounted, ref } from 'vue'
import { getPrinters, printPdf } from 'tauri-plugin-printer-wkhtml-bin'
import { writeFile, remove} from '@tauri-apps/plugin-fs'
import { tempDir, join } from '@tauri-apps/api/path'

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

const printers = ref<any[]>([])

const printDocument = async (printer: string) => {
    // Generate unique temporary filename and full path
    const tempDirPath = await tempDir()
    const tempFileName = `temp_print_${Date.now()}.pdf`
    const fullTempPath = await join(tempDirPath, tempFileName)
    
    try {
        // Convert base64 to Uint8Array and write to temporary file
        const base64Data = props.base64.replace(/^data:application\/pdf;base64,/, '')
        const binaryData = atob(base64Data)
        const bytes = new Uint8Array(binaryData.length)
        for (let i = 0; i < binaryData.length; i++) {
            bytes[i] = binaryData.charCodeAt(i)
        }
        
        await writeFile(fullTempPath, bytes)
        
        const print_settings = {
            id: printer,
            path: fullTempPath,
            printer_setting: printer,
            remove_after_print: false // We'll handle cleanup ourselves
        }

        // Print the temporary file
        await printPdf(print_settings)
    } catch (error) {
        console.error("Print failed", error)
    } finally {
        // Always clean up the temporary file
        try {
            await remove(fullTempPath)
        } catch (cleanupError) {
            console.error("Failed to cleanup temporary file", cleanupError)
        }
    }
}

const initPrinters = async () => {
    const remove_printers = [
        'Fax',
        'Microsoft Print to PDF',
        'Save as PDF',
        'OneNote',
        'OneNote (Desktop)',
        'OneNote for Windows 10',
    ]

    const listed_printers = JSON.parse(await getPrinters())
    console.log(listed_printers)
    printers.value = listed_printers.filter((printer: any) => !remove_printers.includes(printer.Name))
}

onMounted(async () => {
    await initPrinters()
})
</script>

<template>
    <v-menu>
        <template v-slot:activator="{ props }">
            <v-btn variant="text" icon="mdi-printer" v-bind="props" />
      </template>

      <v-list>
        <v-list-item v-for="printer in printers" :key="printer.Name" @click="printDocument(printer.Name)">
          {{ printer.Name }}
        </v-list-item>
      </v-list>
    </v-menu>
</template>

<style scoped>
:deep(.v-list-item__content) {
    padding: 2px 16px !important;
}
</style>
