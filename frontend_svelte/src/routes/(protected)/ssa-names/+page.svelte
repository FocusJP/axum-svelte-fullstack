<script lang="ts">
	import LineChart from '$lib/component/LineChart.svelte';
	import type { GetNameStatsQuery } from '$lib/model/ssa/request';
	import type { TabularDataCollection } from '$lib/model/tabular';
	import { getNameStats } from '$lib/service/ssa';

	let startYear: number = $state(1880);
	let endYear: number = $state(2025);
	let names: string = $state('');

	let namesArray: Array<string> = $derived(
		names
			.split(/[, ]+/)
			.filter((n) => n)
			.map((n) => n.charAt(0).toUpperCase() + n.substring(1).toLowerCase())
	);
	let validFilters = $derived(startYear && endYear && startYear <= endYear && namesArray.length);

	let dataCollection: TabularDataCollection | undefined = $state(undefined);

	const onclick = async () => {
		const query: GetNameStatsQuery = {
			names: namesArray,
			start_year: startYear,
			end_year: endYear
		};

		dataCollection = await getNameStats(query);
	};
</script>

<svelte:head>
	<title>SSA Names</title>
</svelte:head>

<div class="m-4">
	<label class="input me-2 w-3xs">
		<span class="label">Start Year</span>
		<input
			type="number"
			class="validator input"
			required
			placeholder={`Select a start year from 1880 to ${endYear}`}
			min="1880"
			max={endYear}
			title={`Must be between be 1880 and ${endYear}`}
			bind:value={startYear}
		/>
	</label>

	<label class="input me-2 w-3xs">
		<span class="label">End Year</span>
		<input
			type="number"
			class="validator input"
			required
			placeholder={`Select a start year from ${startYear} to 2025`}
			min={startYear}
			max="2025"
			title={`Must be between be ${startYear} and 2025`}
			bind:value={endYear}
		/>
	</label>

	<label class="input me-2 w-xl">
		<span class="label">Names</span>
		<input
			type="text"
			class="input"
			required
			placeholder="Comma or space separated names"
			bind:value={names}
		/>
	</label>

	<button class="btn bg-success" {onclick} disabled={!validFilters}> Apply </button>
</div>

<LineChart {dataCollection} />
