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
    #[serde(rename = "seriesCountByLabelName")]
    pub series_count_by_label_name: Vec<SeriesCount>,
}

#[derive(Debug, Deserialize)]
pub struct SeriesCount {
    pub name: String,
    pub value: i64,
}

