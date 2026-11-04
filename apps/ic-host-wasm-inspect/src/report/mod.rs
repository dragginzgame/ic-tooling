//! Structural facts and exact input identity, without IC cost estimates.

use std::io::{self, Write};

use ic_host_artifacts::{
    artifact::{ArtifactIdentity, Sha256Digest},
    wasm::{InspectionLimits, WasmFacts, inspect},
};

use crate::Error;

pub fn write(
    bytes: &[u8],
    limits: InspectionLimits,
    expected: Option<Sha256Digest>,
    details: bool,
    output: &mut impl Write,
) -> Result<(), Error> {
    // Complete admission before publishing a header or an observed fact.
    let identity = ArtifactIdentity {
        bytes: bytes.len() as u64,
        sha256: Sha256Digest::compute(bytes),
    };
    if let Some(expected) = expected {
        identity.verify(expected).map_err(Error::Input)?;
    }
    let facts = inspect(bytes, limits).map_err(Error::Inspection)?;
    render(output, identity.sha256, facts, details).map_err(Error::Output)
}

fn render(
    output: &mut impl Write,
    digest: Sha256Digest,
    facts: WasmFacts<'_>,
    details: bool,
) -> io::Result<()> {
    writeln!(
        output,
        "sha256\traw_wasm_bytes\tcode_section_bytes\tcode_body_bytes\tdata_section_bytes\tdefined_functions\timported_functions\tdefined_globals\timported_globals\tdata_segments\texports\tcustom_sections"
    )?;
    writeln!(
        output,
        "{digest}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
        facts.raw_bytes,
        facts.code_section_bytes,
        facts.code_body_bytes,
        facts.data_section_bytes,
        facts.defined_functions,
        facts.imported_functions,
        facts.defined_globals,
        facts.imported_globals,
        facts.data_segments,
        facts.exports.len(),
        facts.custom_sections.len()
    )?;
    if details {
        for (name, export) in facts.exports {
            writeln!(
                output,
                "export={name:?} kind={:?} index={}",
                export.kind, export.index
            )?;
        }
        for section in facts.custom_sections {
            writeln!(
                output,
                "custom_section={:?} bytes={}",
                section.name,
                section.data.len()
            )?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
