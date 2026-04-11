import Vision
import Foundation
import CoreGraphics

let args = CommandLine.arguments
guard args.count > 1 else {
    fputs("Usage: ocr <image_path>\n", stderr)
    exit(1)
}
let imagePath = args[1]
let imageURL = URL(fileURLWithPath: imagePath)

guard let imageSource = CGImageSourceCreateWithURL(imageURL as CFURL, nil),
      let cgImage = CGImageSourceCreateImageAtIndex(imageSource, 0, nil) else {
    fputs("Cannot load image: \(imagePath)\n", stderr)
    exit(1)
}

/// OCR a single CGImage tile and return recognized lines
func ocrTile(_ tile: CGImage) -> [String] {
    let semaphore = DispatchSemaphore(value: 0)
    var lines: [String] = []

    let request = VNRecognizeTextRequest { request, error in
        if let observations = request.results as? [VNRecognizedTextObservation] {
            lines = observations.compactMap { $0.topCandidates(1).first?.string }
        }
        semaphore.signal()
    }
    request.recognitionLevel = .accurate
    request.recognitionLanguages = ["zh-Hans", "zh-Hant", "en-US"]
    request.usesLanguageCorrection = true

    let handler = VNImageRequestHandler(cgImage: tile, options: [:])
    do {
        try handler.perform([request])
    } catch {
        fputs("Vision error: \(error.localizedDescription)\n", stderr)
        semaphore.signal()
    }
    semaphore.wait()
    return lines
}

let width = cgImage.width
let height = cgImage.height
let maxTileHeight = 2000  // Split images taller than 2000px into tiles

var allLines: [String] = []

if height <= maxTileHeight {
    // Small image: OCR directly
    allLines = ocrTile(cgImage)
} else {
    // Long image: split into overlapping tiles for better accuracy
    let overlap = 100  // Overlap to avoid cutting text at boundaries
    var y = 0
    while y < height {
        let tileH = min(maxTileHeight, height - y)
        let rect = CGRect(x: 0, y: y, width: width, height: tileH)
        if let tile = cgImage.cropping(to: rect) {
            let tileLines = ocrTile(tile)
            // Deduplicate: skip lines that match the last line from previous tile (overlap region)
            if !allLines.isEmpty && !tileLines.isEmpty {
                // Find where overlap starts — skip duplicate lines
                var startIdx = 0
                for i in 0..<min(5, tileLines.count) {
                    if allLines.last == tileLines[i] {
                        startIdx = i + 1
                        break
                    }
                }
                allLines.append(contentsOf: tileLines[startIdx...])
            } else {
                allLines.append(contentsOf: tileLines)
            }
        }
        y += tileH - overlap
        if tileH < maxTileHeight { break }
    }
}

let resultText = allLines.joined(separator: "\n")
print(resultText)
