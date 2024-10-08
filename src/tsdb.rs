use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TsdbStatus {
    pub status: String,
    pub data: Data,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    //pub head_stats: HeadStats,
    #[serde(rename = "seriesCountByMetricName")]
    pub series_count_by_metric_name: Vec<SeriesCount>,
    #[serde(rename = "labelValueCountByLabelName")]
    pub series_count_by_label_name: Vec<SeriesCount>,
    #[serde(rename = "memoryInBytesByLabelName")]
    pub memory_in_bytes_by_label_name: Vec<SeriesCount>,
}

#[derive(Debug, Deserialize)]
pub struct SeriesCount {
    pub name: String,
    pub value: i64,
}

