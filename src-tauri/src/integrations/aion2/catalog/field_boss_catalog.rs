use super::field_boss_definition::FieldBossDefinition;

pub(crate) struct FieldBossCatalog;

impl FieldBossCatalog {
    // WorldMapFieldNamed slots, matched to NpcData IDs. Only the two home maps
    // supported by WorldBossDecoder are listed. Reference data:
    // https://github.com/taengu/A2Tools-DPS-Meter/blob/main/src-tauri/src/capture/field_bosses.rs
    const VERTERON: [u32; 24] = [
        2100003, 2100040, 2100050, 2100076, 2100077, 2100079, 2100141, 2100177, 2100178, 2100582,
        2100617, 2100661, 2100708, 2100718, 2100876, 2100877, 2100988, 2100989, 2100991, 2101016,
        2101074, 2101120, 2101122, 2101131,
    ];
    const ALTGARD: [u32; 24] = [
        2400017, 2400074, 2400140, 2400141, 2400212, 2400223, 2400274, 2400335, 2400353, 2400358,
        2400419, 2400424, 2400425, 2400474, 2400504, 2400593, 2400607, 2400608, 2400659, 2400709,
        2400800, 2400853, 2400854, 2400855,
    ];

    pub(crate) fn all() -> Vec<FieldBossDefinition> {
        [(1010, Self::VERTERON), (1110, Self::ALTGARD)]
            .into_iter()
            .flat_map(|(map_id, npcs)| {
                npcs.into_iter()
                    .enumerate()
                    .map(move |(index, npc_id)| FieldBossDefinition {
                        id: map_id * 100 + index as u32 + 1,
                        npc_id,
                        map_id,
                    })
            })
            .collect()
    }

    pub(crate) fn find(id: u32) -> Option<FieldBossDefinition> {
        let map_id = id / 100;

        let npcs = match map_id {
            1010 => &Self::VERTERON,
            1110 => &Self::ALTGARD,
            _ => return None,
        };

        let index = (id % 100).checked_sub(1)? as usize;

        Some(FieldBossDefinition {
            id,
            npc_id: *npcs.get(index)?,
            map_id,
        })
    }
}
