use crate::winnowing::{Fingerprint, Location};
use std::collections::HashMap;

// struct Repo(String);

pub fn run(repo_map: HashMap<String, Vec<Fingerprint>>) {
    // step 1: construct inverted index
    let mut inverted_index: HashMap<u64, Vec<Location>> = HashMap::new();
    for fingerprints in repo_map.values() {
        for f in fingerprints {
            let fingerprint_vec = inverted_index.entry(f.hash).or_insert_with(Vec::new);
            fingerprint_vec.push(f.location.clone());
        }
    }
    for i in inverted_index.iter().take(10) {
        println!("{:?}", i);
    }
    println!("{}", inverted_index.keys().len());

    // step 2: construct map from my_locations -> matched_locations
    let mut location_map: HashMap<Location, Vec<Location>> = HashMap::new();
    for (repo, fingerprints) in &repo_map {
        for f in fingerprints {
            let matched_locations = inverted_index.get(&f.hash).unwrap();
            let popularity = matched_locations
                .iter()
                .filter(|&location| location.repo == *repo)
                .count();
            // decide if this is an interesting case or not
            if popularity > 0 && popularity < 1000 {
                let v = location_map
                    .entry(f.location.clone())
                    .or_insert_with(Vec::new);
                v.push(f.location.clone());
            }
        }
    }

    // step 3: O(N^2) comparison
    // for (_, v) in location_map {
    // println!("{:?}", v.len());
    // }
    // naive way to get set of all 'documents' from inverted index
    let documents = inverted_index.values().clone().flatten().map(
        |Location {
             repo,
             commit,
             file,
             line: _,
         }| (repo, commit, file),
    );
    // for each 'document' in documents
    // (d1, d2) => [match1, match2, ...]
    let mut pairs = HashMap::new();
    for (repo, commit, file) in documents {
        // for each doc's locations => matched_locations
        let mut all_matched = vec![];
        for (my_location, matched_locations) in location_map.iter().filter(
            |(
                Location {
                    repo: r,
                    commit: c,
                    file: f,
                    line: _,
                },
                _,
            )| repo == r && commit == c && file == f,
        ) {
            for matched in matched_locations {
                all_matched.push((my_location, matched));
            }
        }
        //println!("{:?}", all_matched.iter().take(10));
        //break;
        for (me, matched) in all_matched {
            // (d, dx) => vec matched
            pairs
                .entry((
                    (me.repo.clone(), me.commit, me.file.clone()),
                    (matched.repo.clone(), matched.commit, matched.file.clone()),
                ))
                .or_insert(vec![])
                .push(matched);
        }
    }
    let mut ranks: Vec<(
        ((String, [u8; 20], String), (String, [u8; 20], String)),
        usize,
    )> = pairs.iter().map(|(k, v)| (k.clone(), v.len())).collect();
    ranks.sort_by(|a, b| a.1.cmp(&b.1));
    println!("{:?}", ranks);
}