use crate::configuration::Configuration;
use crate::error::Error;
use crate::error::Error::NoSalvageContainer;
use crate::{LOG_TARGET, SALVAGE_LABEL};
use bollard::container::{
    ListContainersOptions, RemoveContainerOptions, StartContainerOptions, StopContainerOptions,
};
use bollard::models::ContainerSummary;
use bollard::Docker;
use log::{debug, info, trace, warn};
use std::collections::HashMap;
use std::string::ToString;
use std::time::Instant;

pub async fn post_archive_container_processing(
    container_ids: Option<Vec<String>>,
) -> Result<(), Error> {
    let start_time = Instant::now();
    let docker = connect_docker()?;
    match container_ids {
        None => debug!(target: LOG_TARGET, "No containers to restart"),
        Some(ids) => start_containers(&docker, ids.as_slice()).await?,
    }

    debug!(target: LOG_TARGET, "Post-archive container processing complete after {} milliseconds", start_time.elapsed().as_millis());
    Ok(())
}

/// Run the pre-archive processing on docker containers to identify the Salvage container and its mounts
/// and stop any containers with those mounts. Return container IDs of all containers that were stopped.
pub async fn pre_archive_container_processing(
    config: &Configuration,
) -> Result<Vec<String>, Error> {
    let start_time = Instant::now();
    let docker = connect_docker()?;
    let salvage = find_salvage_container(&docker).await?;
    trace!(target: LOG_TARGET ,"Salvage container: {:?}", salvage);

    let archive_volume_sources = get_archive_volumes(&salvage, config.data_dir.to_string_lossy());
    debug!(target: LOG_TARGET ,"Salvage archive volume sources: {:?}", archive_volume_sources);

    let containers = find_containers_with_mounts(
        &docker,
        archive_volume_sources.as_slice(),
        salvage.id.unwrap_or_default(),
    )
    .await?;
    trace!(target: LOG_TARGET ,"Containers to be shutdown before archive : {:?}", containers);

    let containers = containers
        .into_iter()
        .filter_map(|c| c.id)
        .collect::<Vec<String>>();

    stop_containers(&docker, containers.as_slice()).await?;

    debug!(target: LOG_TARGET, "Pre-archive container processing complete after {} milliseconds", start_time.elapsed().as_millis());
    Ok(containers)
}

fn get_archive_volumes<S: AsRef<str>>(
    container: &ContainerSummary,
    archive_path: S,
) -> Vec<String> {
    trace!(target: LOG_TARGET, "Salvage archive path: {}", archive_path.as_ref());
    trace!(target: LOG_TARGET, "Salvage mounts: {:?}", container.mounts.as_ref().unwrap());
    container
        .mounts
        .clone()
        .unwrap_or_default()
        .into_iter()
        .filter(|m| {
            m.destination
                .clone()
                .unwrap_or_default()
                .starts_with(archive_path.as_ref())
        })
        .filter_map(|m| m.source)
        .collect::<Vec<_>>()
}

/// Find running the salvage containers by label and remove all but the most recent.
async fn find_salvage_container(docker: &Docker) -> Result<ContainerSummary, Error> {
    let list_options = Some(ListContainersOptions {
        filters: HashMap::from([("label", vec![SALVAGE_LABEL])]),
        ..Default::default()
    });
    let containers = docker.list_containers(list_options).await?;

    if containers.len() > 1 {
        info!(target: LOG_TARGET ,"Multiple running Salvage containers found. Removing all but the most recent will be removed");
        let remove_options = Some(RemoveContainerOptions {
            force: true,
            ..Default::default()
        });
        let salvage_container = containers
            .iter()
            .max_by_key(|c| c.created.unwrap_or_default())
            .unwrap();
        trace!(target: LOG_TARGET ,"Most recent Salvage container: {:?}", salvage_container);

        let remove_containers = containers
            .iter()
            .filter(|c| {
                c.id.as_ref()
                    .unwrap_or(&"".to_string())
                    .eq(salvage_container.id.as_ref().unwrap_or(&"".to_string()))
            })
            .collect::<Vec<_>>();
        trace!(target: LOG_TARGET ,"Identified Salvage containers to remove: {:?}", remove_containers);

        for container in remove_containers {
            let remove_result = match container.id.as_ref() {
                None => continue,
                Some(s) => docker.remove_container(s.as_str(), remove_options).await,
            };
            if let Err(error) = remove_result {
                warn!(target: LOG_TARGET ,"Unable to remove container {} because {}", container.id.as_ref().unwrap_or(&"".to_string()), error)
            }
        }
        Ok(salvage_container.clone())
    } else {
        match containers.get(0) {
            None => Err(NoSalvageContainer),
            Some(container) => Ok(container.clone()),
        }
    }
}

/// Find containers with the provided mounts and filter out the Salvage container
async fn find_containers_with_mounts<S: AsRef<str>>(
    docker: &Docker,
    sources: &[String],
    salvage_id: S,
) -> Result<Vec<ContainerSummary>, Error> {
    let list_options = Some(ListContainersOptions::<&str> {
        ..Default::default()
    });
    trace!(target: LOG_TARGET, "Find Mounts ListOptions: {:?}", list_options.as_ref().unwrap());

    let containers = docker
        .list_containers(list_options)
        .await?
        .into_iter()
        .filter(|f| {
            f.id.as_ref()
                .unwrap_or(&"".to_string())
                .as_str()
                .ne(salvage_id.as_ref())
        })
        .collect::<Vec<_>>();

    let containers: Vec<_> = containers
        .into_iter()
        .filter(|c| {
            c.mounts.as_ref().is_some_and(|mounts| {
                mounts
                    .iter()
                    .any(|m| m.source.as_ref().is_some_and(|s| sources.contains(s)))
            })
        })
        .collect();

    Ok(containers)
}

async fn stop_containers<S: AsRef<str>>(docker: &Docker, containers: &[S]) -> Result<(), Error> {
    let stop_options = Some(StopContainerOptions::default());
    for container in containers {
        debug!(target: LOG_TARGET ,"Stopping container: {}", container.as_ref());
        docker
            .stop_container(container.as_ref(), stop_options)
            .await?;
    }
    Ok(())
}

async fn start_containers<S: AsRef<str>>(docker: &Docker, containers: &[S]) -> Result<(), Error> {
    for container in containers {
        let start_options = Some(StartContainerOptions::<&str>::default());
        debug!(target: LOG_TARGET ,"Starting container: {}", container.as_ref());
        docker
            .start_container(container.as_ref(), start_options)
            .await?
    }
    Ok(())
}

fn connect_docker() -> Result<Docker, Error> {
    Ok(Docker::connect_with_socket_defaults()?)
}
