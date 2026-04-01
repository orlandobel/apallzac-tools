<script setup lang="ts">
import { ref } from 'vue'
import LoadedDataTable from '@/components/loaded-data-table/loaded-data-table.vue';

type TabKey = 'datos' | 'previsualizacion'
const activeTab = ref<TabKey>('datos')

</script>

<template>
	<div class="flex h-screen">
		<main class="p0 flex-1">
			<v-tabs v-model="activeTab" background-color="grey.lighten4" grow>
				<v-tab value="datos">Datos</v-tab>
				<v-tab value="previsualizacion">Previsualización</v-tab>
			</v-tabs>

			<v-divider />

			<v-tabs-window class="p-1" v-model="activeTab">
				<v-tabs-window-item value="datos">
					<loaded-data-table />
				</v-tabs-window-item>

				<v-tabs-window-item value="previsualizacion">
					TODO::PDF preview
				</v-tabs-window-item>
			</v-tabs-window>
		</main>

		<aside>

		</aside>
	</div>
</template>

<style scoped>
aside {
	width: 300px;
	border-left: 1px solid #636161;
}
</style>