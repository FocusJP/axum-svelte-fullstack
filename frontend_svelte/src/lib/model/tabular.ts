export type TabularDataRow = { [key: string]: number };

export type TabularDataCollection = {
	properties: Set<string>;
	rows: Array<TabularDataRow>;
};
