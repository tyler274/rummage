use crate::tests::visual_testing::config::{ComparisonMethod, VisualTestConfig};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use std::fs;
use std::path::Path;

/// Comparison result for two images
#[derive(Debug)]
pub struct ComparisonResult {
    /// Number of pixels that differ
    pub pixel_difference_count: usize,
    /// Perceptual hash difference (0.0 to 1.0, lower is more similar)
    pub phash_difference: f32,
    /// Structural similarity score (0.0 to 1.0, higher is more similar)
    pub similarity_score: f32,
    /// Maximum difference in any color channel
    pub max_channel_difference: u8,
    /// Location of biggest difference
    pub max_difference_location: Option<(u32, u32)>,
}

/// Compares two images using the configured method
pub fn compare_images(image1: &DynamicImage, image2: &DynamicImage) -> ComparisonResult {
    let config = VisualTestConfig::default();

    match config.comparison_method {
        ComparisonMethod::PixelPerfect => pixel_perfect_compare(image1, image2),
        ComparisonMethod::PerceptualHash => perceptual_hash_compare(image1, image2),
        ComparisonMethod::SSIM => structural_similarity_compare(image1, image2),
        ComparisonMethod::Combined => combined_compare(image1, image2),
    }
}

/// Pixel-perfect comparison of two images
pub fn pixel_perfect_compare(image1: &DynamicImage, image2: &DynamicImage) -> ComparisonResult {
    // Compare pixels and count differences
    let mut diff_count = 0;
    let mut max_diff = 0;
    let mut max_loc = None;

    // Ensure images are the same size
    if image1.dimensions() != image2.dimensions() {
        return ComparisonResult {
            pixel_difference_count: usize::MAX,
            phash_difference: 1.0,
            similarity_score: 0.0,
            max_channel_difference: 255,
            max_difference_location: None,
        };
    }

    let (width, height) = image1.dimensions();
    let total_pixels = (width * height) as usize;

    // Collect differences
    for y in 0..height {
        for x in 0..width {
            let pixel1 = image1.get_pixel(x, y);
            let pixel2 = image2.get_pixel(x, y);

            // Calculate difference for each channel
            let mut max_channel_diff = 0;
            for i in 0..3 {
                // RGB channels (ignoring alpha)
                let diff = (pixel1[i] as i32 - pixel2[i] as i32).abs() as u8;
                max_channel_diff = max_channel_diff.max(diff);
            }

            if max_channel_diff > 0 {
                diff_count += 1;

                if max_channel_diff > max_diff {
                    max_diff = max_channel_diff;
                    max_loc = Some((x, y));
                }
            }
        }
    }

    ComparisonResult {
        pixel_difference_count: diff_count,
        phash_difference: 0.0, // Not computed in pixel perfect
        similarity_score: 1.0 - (diff_count as f32 / total_pixels as f32),
        max_channel_difference: max_diff,
        max_difference_location: max_loc,
    }
}

/// Perceptual hash comparison of two images
pub fn perceptual_hash_compare(image1: &DynamicImage, image2: &DynamicImage) -> ComparisonResult {
    // Calculate and compare perceptual hashes
    // This is a simplified implementation of perceptual hashing (pHash)

    // Ensure images are comparable
    if image1.dimensions() != image2.dimensions() {
        return ComparisonResult {
            pixel_difference_count: usize::MAX,
            phash_difference: 1.0,
            similarity_score: 0.0,
            max_channel_difference: 255,
            max_difference_location: None,
        };
    }

    // Step 1: Resize both images to a small fixed size (e.g., 32x32)
    // This discards high frequency details and reduces computation
    let size = 32;
    let img1_small = image1.resize_exact(size, size, image::imageops::FilterType::Lanczos3);
    let img2_small = image2.resize_exact(size, size, image::imageops::FilterType::Lanczos3);

    // Step 2: Convert to grayscale
    let img1_gray = img1_small.grayscale();
    let img2_gray = img2_small.grayscale();

    // Step 3: Compute the DCT (Discrete Cosine Transform) - simplified approach
    // In a real implementation, we would use a proper DCT algorithm
    // For this implementation, we'll use average brightness differences as a simpler alternative

    // Calculate average brightness for each image
    let mut img1_values = Vec::new();
    let mut img2_values = Vec::new();

    for y in 0..size {
        for x in 0..size {
            img1_values.push(img1_gray.get_pixel(x, y)[0] as f32);
            img2_values.push(img2_gray.get_pixel(x, y)[0] as f32);
        }
    }

    // Calculate the average brightness
    let avg1: f32 = img1_values.iter().sum::<f32>() / (size * size) as f32;
    let avg2: f32 = img2_values.iter().sum::<f32>() / (size * size) as f32;

    // Step 4: Generate hash by comparing each pixel to the average
    let mut hash1 = Vec::new();
    let mut hash2 = Vec::new();

    for i in 0..(size * size) as usize {
        hash1.push(img1_values[i] >= avg1);
        hash2.push(img2_values[i] >= avg2);
    }

    // Step 5: Calculate Hamming distance (number of bit differences)
    let mut diff_count = 0;
    for i in 0..(size * size) as usize {
        if hash1[i] != hash2[i] {
            diff_count += 1;
        }
    }

    // Normalize the difference (0.0 to 1.0, where 0.0 is identical)
    let phash_diff = diff_count as f32 / (size * size) as f32;
    let similarity = 1.0 - phash_diff;

    // Compute other metrics for consistency
    let pixel_result = pixel_perfect_compare(image1, image2);

    ComparisonResult {
        pixel_difference_count: pixel_result.pixel_difference_count,
        phash_difference: phash_diff,
        similarity_score: similarity,
        max_channel_difference: pixel_result.max_channel_difference,
        max_difference_location: pixel_result.max_difference_location,
    }
}

/// Structural similarity comparison of two images
pub fn structural_similarity_compare(
    image1: &DynamicImage,
    image2: &DynamicImage,
) -> ComparisonResult {
    // Ensure images are the same size
    if image1.dimensions() != image2.dimensions() {
        return ComparisonResult {
            pixel_difference_count: usize::MAX,
            phash_difference: 1.0,
            similarity_score: 0.0,
            max_channel_difference: 255,
            max_difference_location: None,
        };
    }

    // Simplified SSIM implementation
    // Constants for SSIM calculation
    let k1 = 0.01f32;
    let k2 = 0.03f32;
    let l = 255.0f32; // Dynamic range of pixel values
    let c1 = (k1 * l).powi(2);
    let c2 = (k2 * l).powi(2);

    // Convert images to grayscale for SSIM calculation
    let img1_gray = image1.grayscale();
    let img2_gray = image2.grayscale();

    // Calculate mean luminance
    let mut mean1 = 0.0;
    let mut mean2 = 0.0;
    let (width, height) = image1.dimensions();
    let total_pixels = (width * height) as f32;

    for y in 0..height {
        for x in 0..width {
            mean1 += img1_gray.get_pixel(x, y)[0] as f32;
            mean2 += img2_gray.get_pixel(x, y)[0] as f32;
        }
    }
    mean1 /= total_pixels;
    mean2 /= total_pixels;

    // Calculate variance and covariance
    let mut variance1 = 0.0;
    let mut variance2 = 0.0;
    let mut covariance = 0.0;

    for y in 0..height {
        for x in 0..width {
            let val1 = img1_gray.get_pixel(x, y)[0] as f32 - mean1;
            let val2 = img2_gray.get_pixel(x, y)[0] as f32 - mean2;

            variance1 += val1 * val1;
            variance2 += val2 * val2;
            covariance += val1 * val2;
        }
    }

    variance1 /= total_pixels;
    variance2 /= total_pixels;
    covariance /= total_pixels;

    // Calculate SSIM
    let numerator = (2.0 * mean1 * mean2 + c1) * (2.0 * covariance + c2);
    let denominator = (mean1.powi(2) + mean2.powi(2) + c1) * (variance1 + variance2 + c2);
    let ssim = numerator / denominator;

    // Compute other metrics for consistency
    let pixel_result = pixel_perfect_compare(image1, image2);

    ComparisonResult {
        pixel_difference_count: pixel_result.pixel_difference_count,
        phash_difference: 1.0 - ssim, // Convert to same scale as phash_difference
        similarity_score: ssim,
        max_channel_difference: pixel_result.max_channel_difference,
        max_difference_location: pixel_result.max_difference_location,
    }
}

/// Combined comparison using multiple methods
pub fn combined_compare(image1: &DynamicImage, image2: &DynamicImage) -> ComparisonResult {
    // Get results from individual methods
    let pixel_result = pixel_perfect_compare(image1, image2);
    let phash_result = perceptual_hash_compare(image1, image2);
    let ssim_result = structural_similarity_compare(image1, image2);

    // Combine results (weighted average)
    // Weights: SSIM (50%), pHash (30%), Pixel (20%)
    let combined_similarity = (ssim_result.similarity_score * 0.5)
        + ((1.0 - phash_result.phash_difference) * 0.3)
        + (pixel_result.similarity_score * 0.2);

    ComparisonResult {
        pixel_difference_count: pixel_result.pixel_difference_count,
        phash_difference: phash_result.phash_difference,
        similarity_score: combined_similarity,
        max_channel_difference: pixel_result.max_channel_difference,
        max_difference_location: pixel_result.max_difference_location,
    }
}

/// Save a visualization of the differences between two images
pub fn save_difference_visualization(
    image1: &DynamicImage,
    image2: &DynamicImage,
    output_name: &str,
) -> Result<(), String> {
    let config = VisualTestConfig::default();
    let dir = Path::new(&config.artifact_dir);

    if !dir.exists() {
        fs::create_dir_all(dir)
            .map_err(|e| format!("Failed to create artifact directory: {}", e))?;
    }

    // Ensure images are the same size
    if image1.dimensions() != image2.dimensions() {
        return Err("Images have different dimensions".to_string());
    }

    let (width, height) = image1.dimensions();

    // Create a new image to visualize differences
    // We'll create three images side by side: image1, image2, and difference
    let total_width = width * 3;
    let mut diff_image = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(total_width, height);

    // Draw the original images side by side
    for y in 0..height {
        for x in 0..width {
            // Copy first image to left side
            let pixel1 = image1.get_pixel(x, y);
            diff_image.put_pixel(x, y, pixel1);

            // Copy second image to middle
            let pixel2 = image2.get_pixel(x, y);
            diff_image.put_pixel(x + width, y, pixel2);

            // Calculate and visualize difference on right side
            let mut max_diff = 0;
            for i in 0..3 {
                // RGB channels (ignoring alpha)
                let diff = (pixel1[i] as i32 - pixel2[i] as i32).abs() as u8;
                max_diff = max_diff.max(diff);
            }

            // Visualize the difference - use a heat map
            // Red for large differences, green for medium, blue for small
            let scaled_diff = max_diff * 2; // Scale up for better visibility

            // Create a "heat map" visualization:
            // Differences: 0-64 -> Blue to Green transition
            //              64-128 -> Green to Yellow transition
            //              128-255 -> Yellow to Red transition
            let r = if scaled_diff <= 128 {
                if scaled_diff <= 64 {
                    0
                } else {
                    (scaled_diff - 64) * 4
                }
            } else {
                255
            };

            let g = if scaled_diff <= 128 {
                if scaled_diff <= 64 {
                    scaled_diff * 4
                } else {
                    255
                }
            } else {
                255 - ((scaled_diff - 128) * 2)
            };

            let b = if scaled_diff <= 64 {
                255 - (scaled_diff * 4)
            } else {
                0
            };

            diff_image.put_pixel(x + 2 * width, y, Rgba([r, g, b, 255]));
        }
    }

    // Add labels to the image (would be done with an image processing library in a real impl)

    // Save the difference visualization
    let path = dir.join(output_name);
    let dynamic_image = DynamicImage::ImageRgba8(diff_image);
    dynamic_image
        .save(path)
        .map_err(|e| format!("Failed to save difference visualization: {}", e))?;

    Ok(())
}
