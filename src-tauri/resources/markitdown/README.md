# OpenWiki MarkItDown Bundle

This directory is the packaging location for OpenWiki's bundled document
converter.

Run `src-tauri/scripts/setup_markitdown.sh` before creating a release build.
The script builds `resources/markitdown/bin/openwiki-markitdown`, a bundled
converter executable with only the MarkItDown extras needed for PDF, DOCX, and
PPTX imports.

Tauri may place that executable in the final app bundle as either
`markitdown/openwiki-markitdown` or `markitdown/bin/openwiki-markitdown`;
the app checks both locations.
