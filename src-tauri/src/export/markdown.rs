use crate::storage::models::{CapturedContent, ContentType};
use crate::storage::repository::Repository;
use std::fs;
use std::path::{Path, PathBuf};

/// Convert a date string (e.g. "2026-03-19") to a Chinese weekday name.
fn weekday_chinese(date_str: &str) -> String {
    use chrono::NaiveDate;
    match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        Ok(date) => {
            use chrono::Datelike;
            match date.weekday() {
                chrono::Weekday::Mon => "周一".to_string(),
                chrono::Weekday::Tue => "周二".to_string(),
                chrono::Weekday::Wed => "周三".to_string(),
                chrono::Weekday::Thu => "周四".to_string(),
                chrono::Weekday::Fri => "周五".to_string(),
                chrono::Weekday::Sat => "周六".to_string(),
                chrono::Weekday::Sun => "周日".to_string(),
            }
        }
        Err(_) => String::new(),
    }
}

/// Generate markdown content for a single day, grouped by content type.
pub fn generate_day_markdown(
    date: &str,
    contents: &[CapturedContent],
    export_dir: &Path,
) -> Result<String, Box<dyn std::error::Error>> {
    let weekday = weekday_chinese(date);
    let mut md = format!("# {} {}\n\n", date, weekday);

    // Group contents by type
    let texts: Vec<&CapturedContent> = contents
        .iter()
        .filter(|c| matches!(c.content_type, ContentType::Text))
        .collect();
    let urls: Vec<&CapturedContent> = contents
        .iter()
        .filter(|c| matches!(c.content_type, ContentType::Url))
        .collect();
    let images: Vec<&CapturedContent> = contents
        .iter()
        .filter(|c| matches!(c.content_type, ContentType::Image))
        .collect();
    let mixed: Vec<&CapturedContent> = contents
        .iter()
        .filter(|c| matches!(c.content_type, ContentType::Mixed))
        .collect();

    // Text section
    if !texts.is_empty() {
        md.push_str("## 文本\n\n");
        for item in &texts {
            write_content_item(&mut md, item);
        }
    }

    // URL section
    if !urls.is_empty() {
        md.push_str("## 链接\n\n");
        for item in &urls {
            if let Some(url) = &item.source_url {
                md.push_str(&format!("- [{}]({})\n", url, url));
            }
            if let Some(text) = &item.raw_text {
                if !text.is_empty() {
                    // Show a brief preview of the fetched content
                    let preview: String = text.chars().take(200).collect();
                    md.push_str(&format!("  > {}\n", preview.replace('\n', " ")));
                }
            }
            if let Some(note) = &item.user_note {
                if !note.is_empty() {
                    md.push_str(&format!("  **备注**: {}\n", note));
                }
            }
            md.push('\n');
        }
    }

    // Image section
    if !images.is_empty() {
        md.push_str("## 图片\n\n");
        let images_dir = export_dir.join("images");
        let _ = fs::create_dir_all(&images_dir);

        for item in &images {
            if let Some(image_path) = &item.image_path {
                let src = Path::new(image_path);
                if src.exists() {
                    if let Some(filename) = src.file_name() {
                        let dest = images_dir.join(filename);
                        let _ = fs::copy(src, &dest);
                        md.push_str(&format!(
                            "![image](../images/{})\n",
                            filename.to_string_lossy()
                        ));
                    }
                }
            }
            if let Some(text) = &item.raw_text {
                if !text.is_empty() {
                    md.push_str(&format!("> OCR: {}\n", text.replace('\n', " ")));
                }
            }
            if let Some(note) = &item.user_note {
                if !note.is_empty() {
                    md.push_str(&format!("**备注**: {}\n", note));
                }
            }
            md.push('\n');
        }
    }

    // Mixed section
    if !mixed.is_empty() {
        md.push_str("## 其他\n\n");
        for item in &mixed {
            write_content_item(&mut md, item);
        }
    }

    Ok(md)
}

/// Write a single content item to the markdown string.
fn write_content_item(md: &mut String, item: &CapturedContent) {
    let time = extract_time(&item.captured_at);
    md.push_str(&format!("### {} — {}\n\n", time, item.source_app));

    if let Some(text) = &item.raw_text {
        if !text.is_empty() {
            md.push_str(text);
            md.push_str("\n\n");
        }
    }
    if let Some(note) = &item.user_note {
        if !note.is_empty() {
            md.push_str(&format!("> **备注**: {}\n\n", note));
        }
    }
}

/// Extract the HH:MM time portion from an ISO-8601 datetime string.
fn extract_time(datetime: &str) -> String {
    // captured_at is like "2026-03-19T14:30:00+08:00" or "2026-03-19 14:30:00"
    if let Some(t_pos) = datetime.find('T') {
        let time_part = &datetime[t_pos + 1..];
        return time_part.chars().take(5).collect();
    }
    if let Some(space_pos) = datetime.find(' ') {
        let time_part = &datetime[space_pos + 1..];
        return time_part.chars().take(5).collect();
    }
    String::new()
}

/// Export a single day's content to a markdown file.
pub fn export_day(
    date: &str,
    repo: &Repository,
    export_dir: &Path,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let contents = repo.get_content_for_date(date)?;
    if contents.is_empty() {
        return Err(format!("No content found for date {}", date).into());
    }

    let md = generate_day_markdown(date, &contents, export_dir)?;

    // Create month subdirectory, e.g. "2026-03/"
    let month_dir_name = if date.len() >= 7 {
        &date[..7]
    } else {
        date
    };
    let month_dir = export_dir.join(month_dir_name);
    fs::create_dir_all(&month_dir)?;

    let file_path = month_dir.join(format!("{}.md", date));
    fs::write(&file_path, md)?;

    Ok(file_path)
}

/// Export all dates that have content to markdown files.
/// Returns the number of files exported.
pub fn export_all(
    repo: &Repository,
    export_dir: &Path,
) -> Result<usize, Box<dyn std::error::Error>> {
    let dates = repo.get_dates_with_content()?;
    let mut count = 0;

    for (date, _cnt) in &dates {
        match export_day(date, repo, export_dir) {
            Ok(_) => count += 1,
            Err(e) => {
                log::warn!("Failed to export date {}: {}", date, e);
            }
        }
    }

    Ok(count)
}

/// Export content for a date range (inclusive) to markdown files.
/// Returns the number of files exported.
pub fn export_date_range(
    start: &str,
    end: &str,
    repo: &Repository,
    export_dir: &Path,
) -> Result<usize, Box<dyn std::error::Error>> {
    let dates = repo.get_dates_with_content()?;
    let mut count = 0;

    for (date, _cnt) in &dates {
        if date.as_str() >= start && date.as_str() <= end {
            match export_day(date, repo, export_dir) {
                Ok(_) => count += 1,
                Err(e) => {
                    log::warn!("Failed to export date {}: {}", date, e);
                }
            }
        }
    }

    Ok(count)
}
