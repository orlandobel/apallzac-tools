<script setup lang="ts">
defineProps<{
    headers: string[]
    data: any[]
}>()
</script>

<template>
    <table class="table-fixed w-full border border-gray-400 border-collapse">
        <thead>
            <tr>
                <th class="border border-gray-400 py-2" v-for="header in headers" :key="header">
                    {{ header }}
                </th>
            </tr>
        </thead>

        <tbody>
            <tr v-for="row in data" :key="row">
                <td class="border border-gray-400 px-2 py-1" v-for="value in row" :key="value">
                    {{ value }}
                </td>
            </tr>
        </tbody>
    </table>
</template>
