use itertools::Itertools;
use tracing::log;

use crate::bors::{RepositoryClient, RepositoryState};
use crate::github::{LabelModification, LabelTrigger, PullRequestNumber};

/// If there are any label modifications that should be performed on the given PR when `trigger`
/// happens, this function will perform them.
pub async fn handle_label_trigger<Client: RepositoryClient>(
    repo: &mut RepositoryState<Client>,
    pr: PullRequestNumber,
    trigger: LabelTrigger,
) -> anyhow::Result<()> {
    if let Some(modifications) = repo.config.labels.get(&trigger) {
        log::debug!("Performing label modifications {modifications:?}");
        let (add, remove): (Vec<_>, Vec<_>) =
            modifications
                .iter()
                .partition_map(|modification| match modification {
                    LabelModification::Add(label) => itertools::Either::Left(label.clone()),
                    LabelModification::Remove(label) => itertools::Either::Right(label.clone()),
                });
        if !add.is_empty() {
            log::info!("Adding label(s) {add:?}");
            repo.client.add_labels(pr, &add).await?;
        }
        if !remove.is_empty() {
            log::info!("Removing label(s) {remove:?}");
            repo.client.remove_labels(pr, &remove).await?;
        }
    }
    Ok(())
}
