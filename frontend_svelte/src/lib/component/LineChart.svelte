<script lang="ts">
	import {
		AgCharts,
		type AgCartesianSeriesOptions,
		type AgChartInstance,
		type AgChartOptions
	} from 'ag-charts-community';
	import type { TabularDataCollection } from '$lib/model/tabular';
	import { onMount } from 'svelte';

	interface Props {
		dataCollection: TabularDataCollection | undefined;
	}

	let { dataCollection }: Props = $props();

	let container: HTMLDivElement | undefined = $state(undefined);
	let chartApi: AgChartInstance | undefined = $state(undefined);

	const getSeries = () => {
		let properties = [...dataCollection!.properties];

		let series: AgCartesianSeriesOptions[] = properties.map((item) => ({
			type: 'line',
			xKey: 'year',
			xName: 'Year',
			yKey: item,
			yName: item
		}));

		return series;
	};

	$effect(() => {
		if (!dataCollection || !container) return;

		const options: AgChartOptions = {
			container,
			data: dataCollection.rows,
			title: {
				text: 'Birth Name Popularity over Time',
				fontSize: 20
			},
			footnote: {
				text: 'Source: ssa.gov',
				fontSize: 12,
				fontStyle: 'italic'
			},
			legend: {
				enabled: true
			},
			series: getSeries(),
			theme: {
				overrides: {
					line: {
						series: {
							marker: {
								enabled: false
							}
						}
					}
				}
			},
			axes: [
				{
					position: 'bottom',
					type: 'number',
					interval: {
						step: 10
					},
					title: {
						text: 'Birth Year'
					}
				},
				{
					position: 'left',
					type: 'number',
					title: {
						text: 'Birth Count'
					}
				}
			]
		};

		if (!chartApi) {
			chartApi = AgCharts.create(options);
		} else {
			chartApi.update(options);
		}
	});
</script>

<div class="m-10 h-full" bind:this={container}></div>
