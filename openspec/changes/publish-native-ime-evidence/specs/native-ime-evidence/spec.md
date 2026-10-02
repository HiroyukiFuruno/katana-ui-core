## ADDED Requirements

### Requirement: Native producer owns the OS input boundary
KUC SHALL provide a producer that observes native OS IME preedit and commit at its owned window event adapter, before conversion to egui input. The producer MUST capture a non-empty Japanese preedit followed by a Japanese commit and MUST obtain the active input method from the OS. Synthetic RawInput and fixture input MUST NOT be accepted as native input evidence.

#### Scenario: Native composition completes
- **WHEN** an OS IME sends non-empty Japanese preedit followed by commit to the producer window
- **THEN** the producer records the ordered native events and the active input method, and binds them to the root that processes those events

#### Scenario: Synthetic capture is supplied
- **WHEN** a caller supplies fixture data, synthetic RawInput, or only an asserted native flag
- **THEN** the native verifier rejects it even if its internal hashes are consistent

### Requirement: Evidence binds observations to one fresh run
The producer MUST bind revision, platform, fresh run challenge, trusted runner identity, font catalog and font file SHA-256 to one ordered run. The same run MUST supply the scalar sequence U+2B50 U+FE0F, colored RGBA crop observations, non-identity with U+2606, text measurement, caret, hit test, AccessKit text-input observation, and frame/root/receipt hashes.

#### Scenario: Observations originate from one run
- **WHEN** all required observations are captured from the same native run and root
- **THEN** the verifier validates the exact scalar sequence, colored crop, control inequality, measurement and input observations against that run provenance

#### Scenario: Evidence is replayed or mixed
- **WHEN** an artifact has a different revision, platform, challenge, font digest, runner identity, or contains observations from another run or root
- **THEN** verification fails closed and identifies the failed binding

### Requirement: Verifier uses external expected provenance
KUC SHALL expose its verifier through the published registry package. Verification MUST require external expected revision, platform, fresh challenge and trusted producer/runner provenance. Artifact self-declarations and hashes alone MUST NOT establish native origin.

#### Scenario: Registry-only verification
- **WHEN** a foreign consumer depends only on the published KUC registry package and supplies trusted expected provenance
- **THEN** it can verify a canonical native artifact without a repository checkout or unpublished path/git dependency

#### Scenario: Trusted provenance is absent
- **WHEN** the expected native producer or runner provenance cannot be established
- **THEN** verification rejects the artifact instead of accepting its self-declared origin

### Requirement: Native evidence acceptance retains all three operating systems
Completing native evidence acceptance MUST collect and retain verified native evidence for macOS, Windows and Linux at the same revision and publish it as release artifacts. Compilation, synthetic contracts, screenshots and previous run artifacts MUST NOT substitute for these runs. ConsumerArtifactPlanV2 and V1 compatibility MUST be maintained.

The user's 2026-10-02 instruction explicitly authorizes publishing the v0.4.1 hypothesis implementation without available Windows/Linux native environments. This version's tool publication MUST require the existing full quality gates, review, three-OS compile contracts, and revision/version/binary-digest verification of all three tool archives. It MUST declare `native_ime_measured: false`, MUST NOT claim native evidence acceptance, and MUST retain the unfulfilled real-run conditions in Issue #81. This authorization does not waive native evidence verification or permit synthetic data to become native evidence.

#### Scenario: Authorized v0.4.1 hypothesis implementation is published
- **WHEN** the explicitly authorized v0.4.1 implementation passes its full quality, review and three-OS tool distribution gates while native runs remain unavailable
- **THEN** tool and registry publication can proceed with unmeasured status, while native evidence acceptance remains incomplete and Issue #81 stays open

#### Scenario: One OS evidence is missing
- **WHEN** any required OS native run is missing or verification fails
- **THEN** native evidence release acceptance fails and the Issue remains incomplete

#### Scenario: Three OS acceptance succeeds
- **WHEN** all three fresh native runs pass verification at the release revision and existing V1/V2 contracts pass
- **THEN** release artifacts retain their run provenance and the registry-only verifier can validate each artifact
