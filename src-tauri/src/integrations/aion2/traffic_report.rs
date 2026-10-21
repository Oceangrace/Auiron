use super::PacketPipeline;
use crate::core::game::ObservedEventKind;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TrafficReport {
    packets: u64,
    first_packet_us: Option<u64>,
    last_packet_us: Option<u64>,
    resets: u64,
    events: BTreeMap<ObservedEventKind, u64>,
    messages: BTreeMap<u16, u64>,
    rejected: BTreeMap<u16, u64>,
    unknown: BTreeMap<u16, u64>,
    unsupported: BTreeMap<u16, u64>,
    network_errors: u64,
    stream_errors: u64,
    unfinished_bytes: u64,
    last_network_error: Option<String>,
    last_stream_error: Option<String>,
}

impl TrafficReport {
    pub(crate) fn packet(&mut self, timestamp_us: u64, reset: bool) {
        self.packets += 1;
        self.first_packet_us.get_or_insert(timestamp_us);
        self.last_packet_us = Some(timestamp_us);
        self.resets += u64::from(reset);
    }

    pub(crate) fn event(&mut self, kind: ObservedEventKind) {
        *self.events.entry(kind).or_default() += 1;
    }

    pub(crate) fn finish(mut self, pipeline: &PacketPipeline) -> Self {
        self.messages = pipeline.decoder().messages().clone();
        self.rejected = pipeline.decoder().rejected().clone();
        self.unknown = pipeline.decoder().unknown().clone();
        self.unsupported = pipeline.decoder().unsupported().clone();
        self.network_errors = pipeline.network_errors();
        self.stream_errors = pipeline.stream_errors();
        self.unfinished_bytes = pipeline.unfinished_bytes() as u64;
        self.last_network_error = pipeline.last_network_error().map(str::to_owned);
        self.last_stream_error = pipeline.last_stream_error().map(str::to_owned);

        self
    }
}
