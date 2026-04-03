<script setup lang="ts">
import { ref } from 'vue'
import LoadedDataTable from '@/components/loaded-data-table/loaded-data-table.vue';
import previewer from '@/components/previewer/previewer.vue';

type TabKey = 'datos' | 'previsualizacion'
const activeTab = ref<TabKey>('datos')

const headers = [
	'Escuela',
	'Profesor',
	'Alumno',
	'Grado',
	'Talla',
]

const data = [
	{
		'Escuela': 'Escuela 1',
		'Profesor': 'Profesor 1',
		'Alumno': 'Alumno 1',
		'Grado': 'Grado 1',
		'Talla': 'Talla 1'
	},
	{
		'Escuela': 'Escuela 2',
		'Profesor': 'Profesor 2',
		'Alumno': 'Alumno 2',
		'Grado': 'Grado 2',
		'Talla': 'Talla 2'
	},
	{
		'Escuela': 'Escuela 3',
		'Profesor': 'Profesor 3',
		'Alumno': 'Alumno 3',
		'Grado': 'Grado 3',
		'Talla': 'Talla 3'
	}
]

</script>

<template>
	<div class="flex h-screen overflow-hidden">
		<main class="p0 flex-1 flex flex-col h-full">
			<div class="shrink-0 w-full flex">
				<v-tabs v-model="activeTab" background-color="grey.lighten4" grow>
					<v-tab value="datos">Datos</v-tab>
					<v-tab value="previsualizacion">Previsualización</v-tab>
				</v-tabs>

				<div class="flex items-center justify-center w-xs mx-4 px-4 border-l-1 border-l-gray-500">
					<label for="file"
						class="block w-full rounded-lg border-none 
						bg-accent px-3 py-2 text-center text-white cursor-pointer">
						Elegir archivo
					</label>

					<input type="file" name="file"id="file"
						class="sr-only"
						accept="xls, xlsx, ods, gsheet"/>
				</div>
			</div>


			<div class="flex-1 flex overflow-hidden min-h-0 w-full">
				<v-tabs-window class="h-full w-full overflow-auto" v-model="activeTab">
					<v-tabs-window-item class="px-1" value="datos">
						<loaded-data-table :headers="headers" :data="data" />
					</v-tabs-window-item>
	
					<v-tabs-window-item value="previsualizacion" class="relative">
						<previewer />
					</v-tabs-window-item>
				</v-tabs-window>
			</div>
		</main>
	</div>
</template>

<style scoped>
.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}
</style>