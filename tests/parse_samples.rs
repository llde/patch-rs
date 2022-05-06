use std::fs;
use std::path::PathBuf;

use pretty_assertions::assert_eq;

use patch::Patch;

#[test]
fn parse_samples() {
    let samples_path = PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("samples_auto");
    for file in fs::read_dir(samples_path).unwrap() {
        let path = file.unwrap().path();
        if path.extension().unwrap_or_default() != "diff" {
            continue;
        }

        let data = fs::read_to_string(dbg!(&path)).unwrap();
        let patches = Patch::from_multiple(&data)
            .unwrap_or_else(|err| panic!("failed to parse {:?}, error: {}", path, err));

        // Make sure that the patch file we produce parses to the same information as the original
        // patch file.
        let patch_file: String = patches.iter().map(|patch| format!("{}\n", patch)).collect();
        println!("{}", patch_file);
        let patches2 = Patch::from_multiple(&patch_file).unwrap_or_else(|err| {
            panic!(
                "failed to re-parse {:?} after formatting, error: {}",
                path, err
            )
        });
        assert_eq!(patches, patches2);
    }
}

#[test]
fn parse_sample_bzr() {
    let path = PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("samples/bzr.diff");

    let data = fs::read_to_string(dbg!(&path)).unwrap();
    let patches = Patch::from_multiple(&data)
        .unwrap_or_else(|err| panic!("failed to parse {:?}, error: {}", path, err));

    // Make sure that the patch file we produce parses to the same information as the original
    // patch file.
    let patch_file: String = patches.iter().map(|patch| format!("{}\n", patch)).collect();
    println!("{}", patch_file);
    let patches2 = Patch::from_multiple(&patch_file).unwrap_or_else(|err| {
        panic!(
            "failed to re-parse {:?} after formatting, error: {}",
            path, err
        )
    });
    assert_eq!(patches, patches2);
}

#[test]
fn parse_sample_git() {
    let path = PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("samples/git.diff");

    let data = fs::read_to_string(dbg!(&path)).unwrap();
    let patches = Patch::from_multiple(&data)
        .unwrap_or_else(|err| panic!("failed to parse {:?}, error: {}", path, err));

    // Make sure that the patch file we produce parses to the same information as the original
    // patch file.
    let patch_file: String = patches.iter().map(|patch| format!("{}\n", patch)).collect();
    println!("{}", patch_file);
    let patches2 = Patch::from_multiple(&patch_file).unwrap_or_else(|err| {
        panic!(
            "failed to re-parse {:?} after formatting, error: {}",
            path, err
        )
    });
    assert_eq!(patches, patches2);
}

#[test]
fn parse_sample_hg() {
    let path = PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("samples/hg.diff");

    let data = fs::read_to_string(dbg!(&path)).unwrap();
    let patches = Patch::from_multiple(&data)
        .unwrap_or_else(|err| panic!("failed to parse {:?}, error: {}", path, err));

    // Make sure that the patch file we produce parses to the same information as the original
    // patch file.
    let patch_file: String = patches.iter().map(|patch| format!("{}\n", patch)).collect();
    println!("{}", patch_file);
    let patches2 = Patch::from_multiple(&patch_file).unwrap_or_else(|err| {
        panic!(
            "failed to re-parse {:?} after formatting, error: {}",
            path, err
        )
    });
    assert_eq!(patches, patches2);
}

#[test]
fn parse_sample_sample0() {
    let path = PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("samples/sample0.diff");

    let data = fs::read_to_string(dbg!(&path)).unwrap();
    let patches = Patch::from_multiple(&data)
        .unwrap_or_else(|err| panic!("failed to parse {:?}, error: {}", path, err));

    // Make sure that the patch file we produce parses to the same information as the original
    // patch file.
    let patch_file: String = patches.iter().map(|patch| format!("{}\n", patch)).collect();
    println!("{}", patch_file);
    let patches2 = Patch::from_multiple(&patch_file).unwrap_or_else(|err| {
        panic!(
            "failed to re-parse {:?} after formatting, error: {}",
            path, err
        )
    });
    assert_eq!(patches, patches2);
}

#[test]
fn parse_sample_sample1() {
    let path = PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("samples/sample1.diff");

    let data = fs::read_to_string(dbg!(&path)).unwrap();
    let patches = Patch::from_multiple(&data)
        .unwrap_or_else(|err| panic!("failed to parse {:?}, error: {}", path, err));

    // Make sure that the patch file we produce parses to the same information as the original
    // patch file.
    let patch_file: String = patches.iter().map(|patch| format!("{}\n", patch)).collect();
    println!("{}", patch_file);
    let patches2 = Patch::from_multiple(&patch_file).unwrap_or_else(|err| {
        panic!(
            "failed to re-parse {:?} after formatting, error: {}",
            path, err
        )
    });
    assert_eq!(patches, patches2);
}

#[test]
fn parse_sample_sample2() {
    let path = PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("samples/sample2.diff");

    let data = fs::read_to_string(dbg!(&path)).unwrap();
    let patches = Patch::from_multiple(&data)
        .unwrap_or_else(|err| panic!("failed to parse {:?}, error: {}", path, err));

    // Make sure that the patch file we produce parses to the same information as the original
    // patch file.
    let patch_file: String = patches.iter().map(|patch| format!("{}\n", patch)).collect();
    println!("{}", patch_file);
    let patches2 = Patch::from_multiple(&patch_file).unwrap_or_else(|err| {
        panic!(
            "failed to re-parse {:?} after formatting, error: {}",
            path, err
        )
    });
    assert_eq!(patches, patches2);
}

#[test]
fn parse_sample_sample3() {
    let path = PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("samples/sample3.diff");

    let data = fs::read_to_string(dbg!(&path)).unwrap();
    let patches = Patch::from_multiple(&data)
        .unwrap_or_else(|err| panic!("failed to parse {:?}, error: {}", path, err));

    // Make sure that the patch file we produce parses to the same information as the original
    // patch file.
    let patch_file: String = patches.iter().map(|patch| format!("{}\n", patch)).collect();
    println!("{}", patch_file);
    let patches2 = Patch::from_multiple(&patch_file).unwrap_or_else(|err| {
        panic!(
            "failed to re-parse {:?} after formatting, error: {}",
            path, err
        )
    });
    assert_eq!(patches, patches2);
}

#[test]
fn parse_sample_sample4() {
    let path = PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("samples/sample4.diff");

    let data = fs::read_to_string(dbg!(&path)).unwrap();
    let patches = Patch::from_multiple(&data)
        .unwrap_or_else(|err| panic!("failed to parse {:?}, error: {}", path, err));

    // Make sure that the patch file we produce parses to the same information as the original
    // patch file.
    let patch_file: String = patches.iter().map(|patch| format!("{}\n", patch)).collect();
    println!("{}", patch_file);
    let patches2 = Patch::from_multiple(&patch_file).unwrap_or_else(|err| {
        panic!(
            "failed to re-parse {:?} after formatting, error: {}",
            path, err
        )
    });
    assert_eq!(patches, patches2);
}

#[test]
fn parse_sample_sample5() {
    let path = PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("samples/sample5.diff");

    let data = fs::read_to_string(dbg!(&path)).unwrap();
    let patches = Patch::from_multiple(&data)
        .unwrap_or_else(|err| panic!("failed to parse {:?}, error: {}", path, err));

    // Make sure that the patch file we produce parses to the same information as the original
    // patch file.
    let patch_file: String = patches.iter().map(|patch| format!("{}\n", patch)).collect();
    println!("{}", patch_file);
    let patches2 = Patch::from_multiple(&patch_file).unwrap_or_else(|err| {
        panic!(
            "failed to re-parse {:?} after formatting, error: {}",
            path, err
        )
    });
    assert_eq!(patches, patches2);
}
#[test]
fn parse_sample_sample6() {
    let path = PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("samples/sample6.diff");

    let data = fs::read_to_string(dbg!(&path)).unwrap();
    let patches = Patch::from_multiple(&data)
        .unwrap_or_else(|err| panic!("failed to parse {:?}, error: {}", path, err));

    // Make sure that the patch file we produce parses to the same information as the original
    // patch file.
    let patch_file: String = patches.iter().map(|patch| format!("{}\n", patch)).collect();
    println!("{}", patch_file);
    let patches2 = Patch::from_multiple(&patch_file).unwrap_or_else(|err| {
        panic!(
            "failed to re-parse {:?} after formatting, error: {}",
            path, err
        )
    });
    assert_eq!(patches, patches2);
}
#[test]
fn parse_sample_sample7() {
    let path = PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("samples/sample7.diff");

    let data = fs::read_to_string(dbg!(&path)).unwrap();
    let patches = Patch::from_multiple(&data)
        .unwrap_or_else(|err| panic!("failed to parse {:?}, error: {}", path, err));

    // Make sure that the patch file we produce parses to the same information as the original
    // patch file.
    let patch_file: String = patches.iter().map(|patch| format!("{}\n", patch)).collect();
    println!("{}", patch_file);
    let patches2 = Patch::from_multiple(&patch_file).unwrap_or_else(|err| {
        panic!(
            "failed to re-parse {:?} after formatting, error: {}",
            path, err
        )
    });
    assert_eq!(patches, patches2);
}
#[test]
fn parse_sample_svn() {
    let path = PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("samples/svn.diff");

    let data = fs::read_to_string(dbg!(&path)).unwrap();
    let patches = Patch::from_multiple(&data)
        .unwrap_or_else(|err| panic!("failed to parse {:?}, error: {}", path, err));

    // Make sure that the patch file we produce parses to the same information as the original
    // patch file.
    let patch_file: String = patches.iter().map(|patch| format!("{}\n", patch)).collect();
    println!("{}", patch_file);
    let patches2 = Patch::from_multiple(&patch_file).unwrap_or_else(|err| {
        panic!(
            "failed to re-parse {:?} after formatting, error: {}",
            path, err
        )
    });
    assert_eq!(patches, patches2);
}
