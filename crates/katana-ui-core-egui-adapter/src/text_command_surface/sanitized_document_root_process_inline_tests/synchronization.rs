#[test]
fn identity_change_is_rejected_before_revision_policy() -> Result<(), String> {
    let mut process = SanitizedDocumentRootProcess::new(input(3, b"one", "a"))?;

    assert_eq!(
        process.synchronize(input(4, b"two", "b")),
        Err(SanitizedDocumentRootProcessError::IdentityChanged)
    );
    assert_eq!(process.input.snapshot, "a");
    Ok(())
}

#[test]
fn stale_revision_is_rejected() -> Result<(), String> {
    let mut process = SanitizedDocumentRootProcess::new(input(3, b"one", "a"))?;

    assert_eq!(
        process.synchronize(input(2, b"one", "b")),
        Err(SanitizedDocumentRootProcessError::StaleRevision {
            current: 3,
            received: 2,
        })
    );
    assert_eq!(process.input.snapshot, "a");
    Ok(())
}

#[test]
fn same_revision_requires_an_identical_snapshot() -> Result<(), String> {
    let mut process = SanitizedDocumentRootProcess::new(input(3, b"one", "a"))?;

    assert_eq!(
        process.synchronize(input(3, b"one", "b")),
        Err(SanitizedDocumentRootProcessError::RevisionConflict { revision: 3 })
    );
    assert_eq!(process.synchronize(input(3, b"one", "a")), Ok(false));
    Ok(())
}

#[test]
fn same_revision_requires_an_identical_command_projection() -> Result<(), String> {
    let mut process = SanitizedDocumentRootProcess::new(input_with_projection(
        3,
        b"one",
        "a",
        projection("first"),
    ))?;

    assert_eq!(
        process.synchronize(input_with_projection(3, b"one", "a", projection("second"))),
        Err(SanitizedDocumentRootProcessError::RevisionConflict { revision: 3 })
    );
    assert_eq!(
        process.synchronize(input_with_projection(3, b"one", "a", projection("first"))),
        Ok(false)
    );
    Ok(())
}

#[test]
fn same_revision_requires_an_identical_search_projection() -> Result<(), String> {
    let mut process = SanitizedDocumentRootProcess::new(input_with_search_projection(
        3,
        b"one",
        "a",
        search_projection("次へ", 1)?,
    ))?;

    assert_eq!(
        process.synchronize(input_with_search_projection(
            3,
            b"one",
            "a",
            search_projection("次の一致", 2)?,
        )),
        Err(SanitizedDocumentRootProcessError::RevisionConflict { revision: 3 })
    );
    assert_eq!(
        process.synchronize(input_with_search_projection(
            3,
            b"one",
            "a",
            search_projection("次へ", 1)?,
        )),
        Ok(false)
    );
    Ok(())
}

#[test]
fn same_revision_requires_an_identical_context_projection() -> Result<(), String> {
    let mut process = SanitizedDocumentRootProcess::new(input_with_context_projection(
        3,
        b"one",
        "a",
        context_projection("表示", 1),
    ))?;

    assert_eq!(
        process.synchronize(input_with_context_projection(
            3,
            b"one",
            "a",
            context_projection("別の表示", 2),
        )),
        Err(SanitizedDocumentRootProcessError::RevisionConflict { revision: 3 })
    );
    assert_eq!(
        process.synchronize(input_with_context_projection(
            3,
            b"one",
            "a",
            context_projection("表示", 1),
        )),
        Ok(false)
    );
    Ok(())
}

#[test]
fn same_revision_requires_an_identical_tab_projection() -> Result<(), String> {
    let mut process = SanitizedDocumentRootProcess::new(input_with_tab_projection(
        3,
        b"one",
        "a",
        tab_projection("次の文書"),
    ))?;

    assert_eq!(
        process.synchronize(input_with_tab_projection(
            3,
            b"one",
            "a",
            tab_projection("別の文書"),
        )),
        Err(SanitizedDocumentRootProcessError::RevisionConflict { revision: 3 })
    );
    Ok(())
}

#[test]
fn newer_snapshot_is_synchronized_into_the_retained_root() -> Result<(), String> {
    let mut process = SanitizedDocumentRootProcess::new(input(3, b"one", "a"))?;

    assert_eq!(process.synchronize(input(4, b"one", "b")), Ok(true));
    assert_eq!(process.input.revision, 4);
    assert_eq!(process.input.snapshot, "b");
    Ok(())
}
