import { apiUrl } from '$lib/config/api';
import type { GetNameStatsQuery } from '$lib/model/ssa/request';
import type { GetNameStatsResponse } from '$lib/model/ssa/response';
import type { TabularDataCollection, TabularDataRow } from '$lib/model/tabular';
import axios, { type AxiosRequestConfig } from 'axios';

const ssaApiUrl = `${apiUrl}/ssa`;

export async function getNameStats(query: GetNameStatsQuery): Promise<TabularDataCollection> {
	let response: GetNameStatsResponse;

	try {
		response = (await axios.post(`${ssaApiUrl}/name-stats`, query)).data;
	} catch (err) {
		console.error('GetNameStats request failed', err);
		throw err;
	}

	const name_sex_set: Set<string> = new Set();
	const year_row_map: { [year: number]: TabularDataRow } = {};

	response.forEach((item) => {
		const name_sex = `${item.name} (${item.sex})`;
		const row = (year_row_map[item.year] ??= { year: item.year });

		name_sex_set.add(name_sex);
		row[name_sex] = item.count;
	});

	const rows = Object.values(year_row_map);
	rows.sort((a, b) => a.year - b.year);

	const tabular_data: TabularDataCollection = {
		properties: name_sex_set,
		rows
	};

	return tabular_data;
}
