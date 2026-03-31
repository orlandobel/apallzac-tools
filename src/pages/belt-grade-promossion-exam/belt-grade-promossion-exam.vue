<script setup lang="ts">
import { ref } from 'vue'

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

			<v-tabs-items v-model="activeTab">
				<v-tab-item value="datos">
					TODO:: table with loaded dat
				</v-tab-item>

				<v-tab-item value="previsualizacion">
					PDF preview
				</v-tab-item>
			</v-tabs-items>
		</main>

		<aside>

		</aside>
	</div>
</template>

<style scoped>
aside {
	width: 400px;
	border-left: 1px solid #636161;
}
</style>