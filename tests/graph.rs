mod fixture;

use bstr::ByteSlice;

use git_stack::graph::*;

mod test_rebase {
    use super::*;

    #[test]
    fn no_op() {
        let mut repo = git_stack::git::InMemoryRepo::new();
        let plan =
            git_fixture::Dag::load(std::path::Path::new("tests/fixtures/branches.yml")).unwrap();
        fixture::populate_repo(&mut repo, plan);

        let master_branch = repo.find_local_branch("master").unwrap();

        let mut protected_branches = git_stack::git::Branches::default();
        protected_branches.insert(master_branch.clone());

        let mut graph_branches = git_stack::git::Branches::default();
        graph_branches.insert(master_branch.clone());
        graph_branches.insert(repo.find_local_branch("off_master").unwrap());

        let master_commit = repo.find_commit(master_branch.id).unwrap();

        let mut graph = Graph::from_branches(&repo, graph_branches).unwrap();
        git_stack::graph::protect_branches(&mut graph, &repo, &protected_branches);
        git_stack::graph::rebase_branches(&mut graph, master_commit.id);
        let script = git_stack::graph::to_script(&graph);
        dbg!(&script);

        let mut executor = git_stack::git::Executor::new(&repo, false);
        let result = executor.run_script(&mut repo, &script);
        assert_eq!(result, vec![]);
        executor.close(&mut repo, "off_master").unwrap();
        dbg!(&repo);

        let master_branch = repo.find_local_branch("master").unwrap();
        dbg!(&master_branch.id);
        assert_eq!(master_branch.id, master_commit.id);

        let off_master_branch = repo.find_local_branch("off_master").unwrap();
        let ancestors: Vec<_> = repo
            .commits_from(off_master_branch.id)
            .map(|c| c.id)
            .collect();
        dbg!(&ancestors);
        assert!(ancestors.contains(&master_branch.id));
    }

    #[test]
    fn rebase() {
        let mut repo = git_stack::git::InMemoryRepo::new();
        let plan =
            git_fixture::Dag::load(std::path::Path::new("tests/fixtures/branches.yml")).unwrap();
        fixture::populate_repo(&mut repo, plan);

        let master_branch = repo.find_local_branch("master").unwrap();

        let mut protected_branches = git_stack::git::Branches::default();
        protected_branches.insert(master_branch.clone());

        let mut graph_branches = git_stack::git::Branches::default();
        graph_branches.insert(master_branch.clone());
        graph_branches.insert(repo.find_local_branch("feature1").unwrap());
        graph_branches.insert(repo.find_local_branch("feature2").unwrap());

        let master_commit = repo.find_commit(master_branch.id).unwrap();

        let mut graph = Graph::from_branches(&repo, graph_branches).unwrap();
        git_stack::graph::protect_branches(&mut graph, &repo, &protected_branches);
        git_stack::graph::rebase_branches(&mut graph, master_commit.id);
        let script = git_stack::graph::to_script(&graph);
        dbg!(&script);

        let mut executor = git_stack::git::Executor::new(&repo, false);
        let result = executor.run_script(&mut repo, &script);
        assert_eq!(result, vec![]);
        executor.close(&mut repo, "off_master").unwrap();
        dbg!(&repo);

        let master_branch = repo.find_local_branch("master").unwrap();
        dbg!(&master_branch.id);
        assert_eq!(master_branch.id, master_commit.id);

        let feature2_branch = repo.find_local_branch("feature2").unwrap();
        let ancestors: Vec<_> = repo
            .commits_from(feature2_branch.id)
            .map(|c| c.id)
            .collect();
        dbg!(&ancestors);
        assert!(ancestors.contains(&master_branch.id));

        let feature1_branch = repo.find_local_branch("feature1").unwrap();
        dbg!(&feature1_branch.id);
        assert!(ancestors.contains(&feature1_branch.id));
    }
}

mod test_fixup {
    use super::*;

    #[test]
    fn no_op() {
        let mut repo = git_stack::git::InMemoryRepo::new();
        let plan =
            git_fixture::Dag::load(std::path::Path::new("tests/fixtures/branches.yml")).unwrap();
        fixture::populate_repo(&mut repo, plan);

        let master_branch = repo.find_local_branch("master").unwrap();

        let mut protected_branches = git_stack::git::Branches::default();
        protected_branches.insert(master_branch.clone());

        let mut graph_branches = git_stack::git::Branches::default();
        graph_branches.insert(master_branch.clone());
        graph_branches.insert(repo.find_local_branch("off_master").unwrap());

        let master_commit = repo.find_commit(master_branch.id).unwrap();

        let mut graph = Graph::from_branches(&repo, graph_branches).unwrap();
        git_stack::graph::protect_branches(&mut graph, &repo, &protected_branches);
        git_stack::graph::fixup(&mut graph, git_stack::config::Fixup::Move);
        let script = git_stack::graph::to_script(&graph);
        dbg!(&script);

        let mut executor = git_stack::git::Executor::new(&repo, false);
        let result = executor.run_script(&mut repo, &script);
        assert_eq!(result, vec![]);
        executor.close(&mut repo, "off_master").unwrap();
        dbg!(&repo);

        let master_branch = repo.find_local_branch("master").unwrap();
        dbg!(&master_branch.id);
        assert_eq!(master_branch.id, master_commit.id);

        let off_master_branch = repo.find_local_branch("off_master").unwrap();
        let ancestors: Vec<_> = repo
            .commits_from(off_master_branch.id)
            .map(|c| c.id)
            .collect();
        dbg!(&ancestors);
        assert!(ancestors.contains(&master_branch.id));
    }

    #[test]
    fn fixup_move_after_target() {
        let mut repo = git_stack::git::InMemoryRepo::new();
        let plan =
            git_fixture::Dag::load(std::path::Path::new("tests/fixtures/fixup.yml")).unwrap();
        fixture::populate_repo(&mut repo, plan);

        let master_branch = repo.find_local_branch("master").unwrap();

        let mut protected_branches = git_stack::git::Branches::default();
        protected_branches.insert(master_branch.clone());

        let mut graph_branches = git_stack::git::Branches::default();
        graph_branches.insert(master_branch.clone());
        graph_branches.insert(repo.find_local_branch("feature1").unwrap());
        graph_branches.insert(repo.find_local_branch("feature2").unwrap());

        let master_commit = repo.find_commit(master_branch.id).unwrap();

        let mut graph = Graph::from_branches(&repo, graph_branches).unwrap();
        git_stack::graph::protect_branches(&mut graph, &repo, &protected_branches);
        git_stack::graph::fixup(&mut graph, git_stack::config::Fixup::Move);
        let script = git_stack::graph::to_script(&graph);
        dbg!(&script);

        let mut executor = git_stack::git::Executor::new(&repo, false);
        let result = executor.run_script(&mut repo, &script);
        assert_eq!(result, vec![]);
        executor.close(&mut repo, "master").unwrap();
        dbg!(&repo);

        let feature2_branch = repo.find_local_branch("feature2").unwrap();
        let mut commits: Vec<_> = repo
            .commits_from(feature2_branch.id)
            .map(|c| c.summary.to_str_lossy().into_owned())
            .collect();
        commits.reverse();
        assert_eq!(
            commits,
            &[
                "commit 1",
                "commit 2",
                "master commit",
                "feature1 commit 1",
                "fixup! feature1 commit 1",
                "fixup! feature1 commit 1",
                "fixup! feature1 commit 1",
                "feature1 commit 2",
                "fixup! feature1 commit 2",
                "feature1 commit 3",
                "feature2 commit",
            ]
        );

        let master_branch = repo.find_local_branch("master").unwrap();
        dbg!(&master_branch.id);
        assert_eq!(master_branch.id, master_commit.id);

        // feature1 was correctly re-targeted to last fixup
        let feature1_branch = repo.find_local_branch("feature1").unwrap();
        let feature1_commit = repo.find_commit(feature1_branch.id).unwrap();
        assert_eq!(
            feature1_commit.summary.to_str(),
            Ok("fixup! feature1 commit 2")
        );

        // feature2 was correctly re-targeted to last non-fixup
        let feature2_commit = repo.find_commit(feature2_branch.id).unwrap();
        assert_eq!(feature2_commit.summary.to_str(), Ok("feature2 commit"));
    }

    #[test]
    fn stray_fixups() {
        let mut repo = git_stack::git::InMemoryRepo::new();
        let plan =
            git_fixture::Dag::load(std::path::Path::new("tests/fixtures/fixup.yml")).unwrap();
        fixture::populate_repo(&mut repo, plan);

        let master_branch = repo.find_local_branch("feature1").unwrap();

        let mut protected_branches = git_stack::git::Branches::default();
        protected_branches.insert(master_branch.clone());

        let mut graph_branches = git_stack::git::Branches::default();
        graph_branches.insert(master_branch.clone());
        graph_branches.insert(repo.find_local_branch("feature1").unwrap());
        graph_branches.insert(repo.find_local_branch("feature2").unwrap());

        let master_commit = repo.find_commit(master_branch.id).unwrap();

        let mut graph = Graph::from_branches(&repo, graph_branches).unwrap();
        git_stack::graph::protect_branches(&mut graph, &repo, &protected_branches);
        git_stack::graph::fixup(&mut graph, git_stack::config::Fixup::Move);
        let script = git_stack::graph::to_script(&graph);
        dbg!(&script);

        let mut executor = git_stack::git::Executor::new(&repo, false);
        let result = executor.run_script(&mut repo, &script);
        assert_eq!(result, vec![]);
        executor.close(&mut repo, "master").unwrap();
        dbg!(&repo);

        let feature2_branch = repo.find_local_branch("feature2").unwrap();
        let mut commits: Vec<_> = repo
            .commits_from(feature2_branch.id)
            .map(|c| c.summary.to_str_lossy().into_owned())
            .collect();
        commits.reverse();
        assert_eq!(
            commits,
            &[
                "commit 1",
                "commit 2",
                "master commit",
                "feature1 commit 1",
                "feature1 commit 2",
                "fixup! feature1 commit 2",
                "fixup! feature1 commit 1",
                "fixup! feature1 commit 1",
                "fixup! feature1 commit 1",
                "feature1 commit 3",
                "feature2 commit",
            ]
        );

        let master_branch = repo.find_local_branch("feature1").unwrap();
        dbg!(&master_branch.id);
        assert_eq!(master_branch.id, master_commit.id);

        // feature1 was correctly re-targeted to last fixup
        let feature1_branch = repo.find_local_branch("feature1").unwrap();
        let feature1_commit = repo.find_commit(feature1_branch.id).unwrap();
        assert_eq!(feature1_commit.summary.to_str(), Ok("feature1 commit 2"));

        // feature2 was correctly re-targeted to last non-fixup
        let feature2_commit = repo.find_commit(feature2_branch.id).unwrap();
        assert_eq!(feature2_commit.summary.to_str(), Ok("feature2 commit"));
    }
}
