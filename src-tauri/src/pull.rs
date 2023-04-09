use git2::{
    build, AnnotatedCommit, AutotagOption, Config, Error, FetchOptions, Reference, Remote,
    RemoteCallbacks, Repository,
};
use std::io::{self, Write};

pub fn do_fetch<'a>(
    repo: &'a Repository,
    refs: &[&str],
    remote: &'a mut Remote,
) -> Result<AnnotatedCommit<'a>, Error> {
    let mut cb = RemoteCallbacks::new();
    let git_config = Config::open_default().unwrap();
    let mut ch = git2_credentials::CredentialHandler::new(git_config);
    cb.credentials(move |url, username, allowed| ch.try_next_credential(url, username, allowed));
    // Print out our transfer progress.
    cb.transfer_progress(|stats| {
        if stats.received_objects() == stats.total_objects() {
            print!(
                "Resolving deltas {}/{}\r",
                stats.indexed_deltas(),
                stats.total_deltas()
            );
        } else if stats.total_objects() > 0 {
            print!(
                "Received {}/{} objects ({}) in {} bytes\r",
                stats.received_objects(),
                stats.total_objects(),
                stats.indexed_objects(),
                stats.received_bytes()
            );
        }
        io::stdout().flush().unwrap();
        true
    });

    let mut fo = FetchOptions::new();
    fo.remote_callbacks(cb);
    // Always fetch all tags.
    // Perform a download and also update tips
    fo.download_tags(AutotagOption::All);
    println!("Fetching {} for repo", remote.name().unwrap());
    remote.fetch(refs, Some(&mut fo), None)?;

    // If there are local objects (we got a thin pack), then tell the user
    // how many objects we saved from having to cross the network.
    let stats = remote.stats();
    if stats.local_objects() > 0 {
        println!(
            "\rReceived {}/{} objects in {} bytes (used {} local \
             objects)",
            stats.indexed_objects(),
            stats.total_objects(),
            stats.received_bytes(),
            stats.local_objects()
        );
    } else {
        println!(
            "\rReceived {}/{} objects in {} bytes",
            stats.indexed_objects(),
            stats.total_objects(),
            stats.received_bytes()
        );
    }

    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    Ok(repo.reference_to_annotated_commit(&fetch_head)?)
}

pub fn fast_forward(
    repo: &Repository,
    reference: &mut Reference,
    fetch_commit: &AnnotatedCommit,
) -> Result<(), Error> {
    let name = match reference.name() {
        Some(s) => s.to_string(),
        None => String::from_utf8_lossy(reference.name_bytes()).to_string(),
    };

    let msg = format!(
        "Fast-Forward: Setting {} to id: {}",
        name,
        fetch_commit.id()
    );
    println!("{}", msg);
    reference.set_target(fetch_commit.id(), &msg)?;
    repo.set_head(&name)?;
    repo.checkout_head(Some(build::CheckoutBuilder::default().force()))?;
    Ok(())
}

pub fn normal_merge(
    repo: &Repository,
    local: &AnnotatedCommit,
    remote: &AnnotatedCommit,
) -> Result<(), Error> {
    let local_tree = repo.find_commit(local.id())?.tree()?;
    let remote_tree = repo.find_commit(remote.id())?.tree()?;
    let ancestor = repo
        .find_commit(repo.merge_base(local.id(), remote.id())?)?
        .tree()?;
    let mut idx = repo.merge_trees(&ancestor, &local_tree, &remote_tree, None)?;

    if idx.has_conflicts() {
        println!("Merge conflicts detected...");
        repo.checkout_index(Some(&mut idx), None)?;
        return Ok(());
    }
    let result_tree = repo.find_tree(idx.write_tree_to(repo)?)?;
    // now create the merge commit
    let msg = format!("Merge: {} into {}", remote.id(), local.id());
    let sig = repo.signature()?;
    let local_commit = repo.find_commit(local.id())?;
    let remote_commit = repo.find_commit(remote.id())?;
    // Do our merge commit and set current branch head to that commit.
    let _merge_commit = repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        &msg,
        &result_tree,
        &[&local_commit, &remote_commit],
    )?;
    // Set working tree to match head.
    repo.checkout_head(None)?;
    Ok(())
}

pub fn do_merge<'a>(
    repo: &'a Repository,
    remote_branch: &str,
    fetch_commit: AnnotatedCommit<'a>,
) -> Result<(), Error> {
    // 1. do a merge analysis
    let (merge_analysis, _) = repo.merge_analysis(&[&fetch_commit])?;
    // 2. Do the appropriate merge
    if merge_analysis.is_fast_forward() {
        println!("Doing a fast forward");
        // do a fast forward
        let refname = format!("refs/heads/{}", remote_branch);
        match repo.find_reference(&refname) {
            Ok(mut r) => {
                fast_forward(repo, &mut r, &fetch_commit)?;
            }
            Err(_) => {
                // The branch doesn't exist so just set the reference to the
                // commit directly. Usually this is because you are pulling
                // into an empty repository.
                repo.reference(
                    &refname,
                    fetch_commit.id(),
                    true,
                    &format!("Setting {} to {}", remote_branch, fetch_commit.id()),
                )?;
                repo.set_head(&refname)?;
                repo.checkout_head(Some(
                    build::CheckoutBuilder::default()
                        .allow_conflicts(true)
                        .conflict_style_merge(true)
                        .force(),
                ))?;
            }
        };
        return Ok(());
    }
    if merge_analysis.is_normal() {
        // do a normal merge
        let head_commit = repo.reference_to_annotated_commit(&repo.head()?)?;
        normal_merge(&repo, &head_commit, &fetch_commit)?;
        return Ok(());
    }
    println!("Already up to date");

    return Ok(());
}
