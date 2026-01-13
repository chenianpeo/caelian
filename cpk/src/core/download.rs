pub fn download_file(url: &str, path: &std::path::Path) -> Result<(), String> {
    let resp = ureq::get(url).call().map_err(|e| e.to_string())?;

    let mut reader = resp.into_reader(); // ureq 2.x，需要 import std::io::Read
    let mut out = std::fs::File::create(path).map_err(|e| e.to_string())?;
    std::io::copy(&mut reader, &mut out).map_err(|e| e.to_string())?;
    Ok(())
}