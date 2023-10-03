#!/bin/bash

# Create the main directory
mkdir rust-Exploritorium 
cd rust-advanced-roadmap

# Create the main README.md
cat <<EOL > README.md
# Rust Advanced Roadmap

This repository contains a comprehensive roadmap for studying advanced Rust programming. It is designed to help learners progress from basic to advanced topics in Rust.

## Roadmap Contents

- [Preliminary Concepts](#01-preliminary-concepts)
- [Intermediate Concepts](#02-intermediate-concepts)
- [Advanced Concepts](#03-advanced-concepts)
- [Advanced Language Features](#04-advanced-language-features)
- [Advanced Tooling and Ecosystem](#05-advanced-tooling-and-ecosystem)
- [Systems Programming](#06-systems-programming)
- [Web Development (Optional)](#07-web-development-optional)
- [Data Science and Machine Learning (Optional)](#08-data-science-and-machine-learning-optional)
- [Embedded and IoT (Optional)](#09-embedded-and-iot-optional)
- [Contributing to Open Source](#10-contributing-to-open-source)
- [Advanced Topics (Optional)](#11-advanced-topics-optional)
- [Continuous Learning](#12-continuous-learning)

...

## Contribution Guidelines

Feel free to contribute to this roadmap by opening issues or pull requests to suggest improvements or add resources.
EOL

# Create code and resources directories
mkdir code resources

# Create directories for each section
for i in {01..12}; do
    section_name=""
    case $i in
        01) section_name="preliminary_concepts";;
        02) section_name="intermediate_concepts";;
        03) section_name="advanced_concepts";;
        04) section_name="advanced_language_features";;
        05) section_name="advanced_tooling_and_ecosystem";;
        06) section_name="systems_programming";;
        07) section_name="web_development_optional";;
        08) section_name="data_science_and_ml_optional";;
        09) section_name="embedded_and_iot_optional";;
        10) section_name="contributing_to_open_source";;
        11) section_name="advanced_topics_optional";;
        12) section_name="continuous_learning";;
    esac

    mkdir -p "code/$section_name"
    mkdir -p "resources/$section_name"
    
    # Create README.md for each section
    cat <<EOL > "code/$section_name/README.md"
# $section_name

This directory contains code examples and projects related to the "$section_name" section of the Rust Advanced Roadmap.

...
EOL

    # Create README.md for each resources section
    cat <<EOL > "resources/$section_name/README.md"
# $section_name Resources

This directory contains supplementary materials and resources related to the "$section_name" section of the Rust Advanced Roadmap.

...
EOL

done

# Create the remaining files and directories (LICENSE, CONTRIBUTING.md, etc.)
touch LICENSE CONTRIBUTING.md CHANGELOG.md ISSUE_TEMPLATE.md PULL_REQUEST_TEMPLATE.md

echo "Rust- Exploritorium created successfully!"
