extern crate chrono;
extern crate lablog_store as store;
extern crate tempdir;

use self::chrono::prelude::*;
use self::tempdir::TempDir;
use CSVStore;
use std::collections::BTreeSet;
use store::note::Note;
use store::note::Notes;
use store::project::Project;
use store::project::Projects;
use store::store::Store;

#[test]
fn write_read_notes() {
    let data_dir = TempDir::new("lablog_store_csv_test_write_read_note").expect("can not create temporary directory for test");
    let teststore = CSVStore::new(data_dir.path().to_path_buf());
    let testname = "test".into();

    let mut notes = Notes::new();

    for i in 0..100 {
        let note = Note {
            time_stamp: Utc::now(),
            value: "test".to_string() + &i.to_string(),
        };

        teststore
            .write_note(&testname, &note)
            .expect("can not write note to store");

        notes.insert(note);
    }

    let testproject = Project {
        archived: false,
        name: testname.clone(),
        notes: notes,
    };

    let storeproject = teststore
        .get_project(testname, false)
        .expect("can not get note from store");

    println!("storeproject: {:#?}", storeproject);

    if testproject.notes.len() != storeproject.notes.len() {
        panic!(
            "storenotes length ({}) is different from notes length ({})",
            testproject.notes.len(),
            storeproject.notes.len()
        )
    }

    assert_eq!(testproject, storeproject);
}

#[test]
#[should_panic]
fn write_empty_note() {
    let data_dir = TempDir::new("lablog_store_csv_test_write_read_note").expect("can not create temporary directory for test");
    let teststore = CSVStore::new(data_dir.path().to_path_buf());
    let testproject = "test".into();

    let note = Note {
        time_stamp: Utc::now(),
        value: "".to_string(),
    };

    teststore
        .write_note(&testproject, &note)
        .expect("can not write note to store");
}

#[test]
fn get_projects_list() {
    let data_dir = TempDir::new("lablog_store_csv_test_write_read_note").expect("can not create temporary directory for test");
    let teststore = CSVStore::new(data_dir.path().to_path_buf());

    let note = Note {
        time_stamp: Utc::now(),
        value: "test".to_string(),
    };

    let mut list = BTreeSet::new();

    for i in 1..100 {
        let testproject = format!("test{}", i).into();

        teststore
            .write_note(&testproject, &note)
            .expect("can not write note to store");

        list.insert(testproject);
    }

    let storelist = teststore
        .get_projects_list()
        .expect("can not get project list from store");

    println!("storelist: {:#?}", storelist);

    if list.len() != storelist.len() {
        panic!(
            "storelist length ({}) is not list length ({})",
            storelist.len(),
            list.len()
        )
    }

    assert_eq!(list, storelist);
}

#[test]
fn project_name_from_path() {
    let data_dir = TempDir::new("lablog_store_csv_test_write_read_note").expect("can not create temporary directory for test");
    let teststore = CSVStore::new(data_dir.path().to_path_buf());
    {
        let expected = "test".into();
        let path = teststore.project_path(&expected);

        let got = teststore
            .project_name_from_path(&path)
            .expect("can not get project name from path");

        assert_eq!(expected, got);
    }

    {
        let expected = "test.test".into();
        let path = teststore.project_path(&expected);

        let got = teststore
            .project_name_from_path(&path)
            .expect("can not get project name from path");

        assert_eq!(expected, got);
    }

    {
        let expected = "test.test.test".into();
        let path = teststore.project_path(&expected);

        let got = teststore
            .project_name_from_path(&path)
            .expect("can not get project name from path");

        assert_eq!(expected, got);
    }
}

#[test]
fn get_projects() {
    let data_dir = TempDir::new("lablog_store_csv_test_get_projects").expect("can not create temporary directory for test");
    let teststore = CSVStore::new(data_dir.path().to_path_buf());

    let note = Note {
        time_stamp: Utc::now(),
        value: "test".to_string(),
    };

    let mut notes = Notes::new();
    notes.insert(note.clone());

    let mut testprojects = Projects::new();

    for i in 1..100 {
        let testproject = format!("test{}", i).into();

        teststore
            .write_note(&testproject, &note)
            .expect("can not write note to store");

        testprojects.insert(Project {
            name: testproject,
            notes: notes.clone(),
            archived: false,
        });
    }

    let storeprojects = teststore
        .get_projects()
        .expect("can not get projects from store");

    println!("storeprojects: {:#?}", storeprojects);

    if testprojects.len() != storeprojects.len() {
        panic!(
            "storeprojects length ({}) is not testprojects length ({})",
            storeprojects.len(),
            testprojects.len()
        )
    }

    assert_eq!(testprojects, storeprojects);
}

#[test]
fn archive_project() {
    let data_dir = TempDir::new("lablog_store_csv_test_get_projects").expect("can not create temporary directory for test");
    let teststore = CSVStore::new(data_dir.path().to_path_buf());
    let testname = "test".into();

    let note = Note {
        time_stamp: Utc::now(),
        value: "test".to_string(),
    };

    teststore
        .write_note(&testname, &note)
        .expect("can not write note to store");

    let mut notes = Notes::new();
    notes.insert(note);

    teststore
        .archive_project(&testname)
        .expect("can not archive project");

    let testproject = Project {
        archived: true,
        name: testname.clone(),
        notes: notes,
    };

    let storeproject = teststore
        .get_project(testname, true)
        .expect("can not get project from store");

    assert_eq!(testproject, storeproject);
}

#[test]
fn archive_project_merging() {
    let data_dir = TempDir::new("lablog_store_csv_test_get_projects").expect("can not create temporary directory for test");
    let teststore = CSVStore::new(data_dir.path().to_path_buf());
    let testname = "test".into();

    let note = Note {
        time_stamp: Utc::now(),
        value: "test".to_string(),
    };

    let note2 = Note {
        time_stamp: Utc::now(),
        value: "test2".to_string(),
    };

    teststore
        .write_note(&testname, &note)
        .expect("can not write note to store");

    teststore
        .archive_project(&testname)
        .expect("can not archive project");

    teststore
        .write_note(&testname, &note2)
        .expect("can not write note to store");

    teststore
        .archive_project(&testname)
        .expect("can not archive project");

    let mut notes = Notes::new();
    notes.insert(note);
    notes.insert(note2);

    let testproject = Project {
        archived: true,
        name: testname.clone(),
        notes: notes,
    };

    let storeproject = teststore
        .get_project(testname, true)
        .expect("can not get project from store");

    assert_eq!(testproject, storeproject);
}
