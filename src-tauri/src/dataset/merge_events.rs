use crate::dataset::spot::AllRequirements;

use super::spot::AnyOfAllRequirements;

#[derive(Clone)]
pub struct Event {
    pub name: String,
    pub requirements: AnyOfAllRequirements,
}

pub fn merge_events(requirements: AnyOfAllRequirements, events: &[Event]) -> AnyOfAllRequirements {
    // [['event:a', 'event:b', 'c']]
    // 'event:a': [['d', 'e', 'f']]
    // 'event:b': [['g', 'h'], ['i', 'j']]
    // ↓
    // [['event:b', 'c', 'd', 'e', 'f']]
    // ↓
    // [
    //   ['c', 'd', 'e', 'f', 'g', 'h']
    //   ['c', 'd', 'e', 'f', 'i', 'j']
    // ]
    let mut current = requirements;
    for event in events {
        if current
            .0
            .iter()
            .all(|target_group| !target_group.0.iter().any(|x| x.get() == event.name))
        {
            continue;
        }
        current = AnyOfAllRequirements(
            current
                .0
                .into_iter()
                .flat_map(|target_group| -> Vec<AllRequirements> {
                    if !target_group.0.iter().any(|x| x.get() == event.name) {
                        return vec![target_group];
                    }
                    event
                        .requirements
                        .0
                        .iter()
                        .map(|event_group| -> AllRequirements {
                            AllRequirements(
                                event_group
                                    .0
                                    .clone()
                                    .into_iter()
                                    .chain(
                                        target_group
                                            .0
                                            .clone()
                                            .into_iter()
                                            .filter(|x| {
                                                x.get() != event.name
                                                    && !event_group.0.iter().any(|y| y == x)
                                            })
                                            .collect::<Vec<_>>(),
                                    )
                                    .collect(),
                            )
                        })
                        .collect()
                })
                .collect(),
        );
    }
    current
}
