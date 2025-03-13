# GitHub Workflows for Rummage

This directory contains GitHub Actions workflows for the Rummage project.

## Workflows

### Deploy mdBook Documentation

The `deploy-docs.yml` workflow automatically builds and deploys the project documentation to GitHub Pages whenever changes are made to the `docs/` directory on the main branch.

## GitHub Pages Setup

To complete the GitHub Pages setup:

1. Go to your GitHub repository settings
2. Navigate to the "Pages" section
3. In the "Build and deployment" section:
   - Set "Source" to "GitHub Actions"
4. Save the changes

Once configured, your documentation will be available at `https://tyler274.github.io/rummage/`. 