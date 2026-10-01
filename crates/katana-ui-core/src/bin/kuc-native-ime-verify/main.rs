use katana_ui_core::native_ime_evidence::{
    NativeImeEvidenceArtifact, NativeImeVerificationExpectations,
};
use std::{error::Error, path::Path, process::ExitCode};

fn main() -> ExitCode {
    match verify_arguments() {
        Ok(()) => {
            println!("native IME evidence verified against supplied trusted expectations");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("kuc-native-ime-verify: {error}");
            ExitCode::FAILURE
        }
    }
}

fn verify_arguments() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let [artifact_path, expectations_path] = args.as_slice() else {
        return Err("usage: kuc-native-ime-verify ARTIFACT.json TRUSTED_EXPECTATIONS.json".into());
    };
    let artifact_path = Path::new(artifact_path).canonicalize()?;
    let expectations_path = Path::new(expectations_path).canonicalize()?;
    if artifact_path == expectations_path {
        return Err("trusted expectations must be supplied independently of the artifact".into());
    }
    let artifact: NativeImeEvidenceArtifact =
        serde_json::from_slice(&std::fs::read(artifact_path)?)?;
    let expected: NativeImeVerificationExpectations =
        serde_json::from_slice(&std::fs::read(expectations_path)?)?;
    artifact.verify(&expected)?;
    Ok(())
}
