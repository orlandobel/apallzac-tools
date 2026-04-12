<script setup lang="ts">
import Candidate from '@/types/Candidate.type';

const props = defineProps<{
    headers: string[]
    disabled: boolean
    data: Candidate[]
    onGenerateClick: () => void
}>()
</script>

<template>
    <div>
        <table class="table-fixed w-full border border-gray-400 border-collapse">
            <thead>
                <tr>
                    <th class="border border-gray-400 py-2" v-for="header in headers" :key="header">
                        {{ header }}
                    </th>
                </tr>
            </thead>
    
            <tbody>
                <tr v-for="(row, index) in data" :key="index">
                    <td class="border border-gray-400 px-2 py-1">
                        {{ row.school }}
                    </td>
                    <td class="border border-gray-400 px-2 py-1">
                        {{ row.trainer }}
                    </td>
                    <td class="border border-gray-400 px-2 py-1">
                        {{ row.name }}
                    </td>
                    <td class="border border-gray-400 px-2 py-1">
                        {{ row.belt }}
                    </td>
                    <td class="border border-gray-400 px-2 py-1">
                        {{ row.belt_size }}
                    </td>
                </tr>
            </tbody>
        </table>

        <button class="absolute bg-primary disabled:opacity-50  rounded-lg bottom-0 right-0 py-2 px-4 my-2 mx-4" :disabled="disabled" @click="onGenerateClick">
            Generar PDF
        </button>
    </div>
</template>
