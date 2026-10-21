use super::widget_replay_report::WidgetReplayReport;
use crate::config::ApplicationConfiguration;
use crate::core::game::SourceFrame;
use crate::core::game::SourceStatus;
use crate::core::geometry::Viewport;
use crate::core::localization::LocaleRegistry;
use crate::integrations::aion2::PacketPipeline;
use crate::integrations::aion2::capture::PcapReader;
use crate::integrations::aion2::catalog::CatalogStore;
use crate::integrations::aion2::state::GameState;
use crate::integrations::windows::WindowsTextMatcher;
use crate::modules::interfaces::InterfaceRepository;
use crate::modules::interfaces::InterfaceService;
use crate::modules::interfaces::LiveFrame;
use crate::modules::interfaces::PreviewText;
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;

pub(crate) struct ReplayInspection;

impl ReplayInspection {
    pub(crate) fn run(recording: &Path, port: u16, workspace: &Path) -> Result<String, String> {
        if port == 0 {
            return Err("Server port must be positive".into());
        }

        let configuration = ApplicationConfiguration::load()?;

        let mut interfaces = Self::interfaces(&configuration, workspace)?;

        let mut pipeline = PacketPipeline::new(
            configuration.protocol()?,
            configuration.npc_index()?,
            configuration.runtime_catalog()?,
        );

        let mut state = GameState::new(
            uuid::Uuid::new_v4().to_string(),
            pipeline.capabilities().clone(),
            pipeline.catalog().clone(),
            configuration.combat_idle_ms(),
        );

        let mut input = PcapReader::open(recording)?;

        let mut next_sample = 0;

        let mut now = 0;

        let mut reports = BTreeMap::<String, WidgetReplayReport>::new();

        let mut visible_frames = BTreeMap::<String, u64>::new();

        let mut timed_frames = BTreeMap::<String, u64>::new();

        let mut maximum_instances = BTreeMap::<String, usize>::new();

        while let Some(packet) = input.next()? {
            now = packet.timestamp_us();

            let update = pipeline.push(&packet, &[port]);

            for event in update.events() {
                state.apply(event, now)?;
            }

            if let Some(reset) = update.reset() {
                state.recover(reset);
            }

            if now < next_sample {
                continue;
            }

            next_sample = now.saturating_add(configuration.capture().publish_ms() * 1000);

            let source = SourceFrame::new(state.snapshot(now, SourceStatus::Replay), None);

            let frame = interfaces.live(&source, PreviewText::default())?;

            for (id, visual) in frame.visuals() {
                let maximum = maximum_instances.entry(id.clone()).or_default();

                *maximum = (*maximum).max(visual.visible_instances());
            }

            Self::tally(&frame, &mut reports, &mut visible_frames, &mut timed_frames);
        }

        let snapshot = state.snapshot(now, SourceStatus::Replay);

        let source = SourceFrame::new(snapshot.clone(), None);

        let final_frame = interfaces.live(&source, PreviewText::default())?;

        serde_json::to_string_pretty(&serde_json::json!({
            "maximumVisibleInstances": maximum_instances, "finalVisuals": final_frame.visuals(),
            "widgets": reports, "visibleFrames": visible_frames, "timedFrames": timed_frames,
            "networkErrors": pipeline.network_errors(), "streamErrors": pipeline.stream_errors(),
            "rejected": pipeline.decoder().rejected(), "unsupported": pipeline.decoder().unsupported(),
            "finalSnapshot": snapshot,
        })).map_err(|error| error.to_string())
    }

    fn interfaces(
        configuration: &ApplicationConfiguration,
        workspace: &Path,
    ) -> Result<InterfaceService, String> {
        let zones = configuration.zones()?;

        let locales = Arc::new(LocaleRegistry::new(
            configuration.packs()?,
            configuration.localization().fallback_locale().to_owned(),
            configuration.localization().default_locale().to_owned(),
            Vec::new(),
            zones.values(),
        )?);

        let catalog = CatalogStore::load(
            configuration.manifest(),
            configuration.player(),
            configuration.enemy(),
            configuration.catalog(),
            locales,
            Arc::new(WindowsTextMatcher::new()),
            configuration.runtime_catalog()?,
        )?;

        let mut service = InterfaceService::new(
            InterfaceRepository::new(workspace.to_path_buf()),
            catalog,
            Viewport {
                width: 1920.0,
                height: 1080.0,
            },
            zones,
        );

        service.start();
        service.validate_loaded()?;

        Ok(service)
    }

    fn tally(
        frame: &LiveFrame,
        reports: &mut BTreeMap<String, WidgetReplayReport>,
        visible: &mut BTreeMap<String, u64>,
        timed: &mut BTreeMap<String, u64>,
    ) {
        for (id, state) in frame.states() {
            reports.entry(id.clone()).or_default().observe(*state);
        }

        for (id, visual) in frame.visuals() {
            if visual.visible_instances() > 0 {
                *visible.entry(id.clone()).or_default() += 1;
            }

            if visual.remaining().is_some() {
                *timed.entry(id.clone()).or_default() += 1;
            }
        }
    }
}
