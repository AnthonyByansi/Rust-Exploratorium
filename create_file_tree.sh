#!/bin/bash

# Create the directories for each section
for i in {01..12}; do
  mkdir "$i"_preliminary_concepts
  mkdir "$i"_intermediate_concepts
  mkdir "$i"_advanced_concepts
  mkdir "$i"_advanced_language_features
  mkdir "$i"_advanced_tooling_and_ecosystem
  mkdir "$i"_systems_programming
  mkdir "$i"_web_development_optional
  mkdir "$i"_data_science_and_ml_optional
  mkdir "$i"_embedded_and_iot_optional
  mkdir "$i"_contributing_to_open_source
  mkdir "$i"_advanced_topics_optional
  mkdir "$i"_continuous_learning
done

# Create the 'resources' directory
mkdir resources

# Create the directories for resources in each section
for i in {01..12}; do
  mkdir resources/"$i"_preliminary_concepts
  mkdir resources/"$i"_intermediate_concepts
  mkdir resources/"$i"_advanced_concepts
  mkdir resources/"$i"_advanced_language_features
  mkdir resources/"$i"_advanced_tooling_and_ecosystem
  mkdir resources/"$i"_systems_programming
  mkdir resources/"$i"_web_development_optional
  mkdir resources/"$i"_data_science_and_ml_optional
  mkdir resources/"$i"_embedded_and_iot_optional
  mkdir resources/"$i"_contributing_to_open_source
  mkdir resources/"$i"_advanced_topics_optional
  mkdir resources/"$i"_continuous_learning
done

# Create additional files and directories
touch README.md
touch LICENSE
touch CONTRIBUTING.md
touch CHANGELOG.md
touch ISSUE_TEMPLATE.md
touch PULL_REQUEST_TEMPLATE.md

# Print a message indicating completion
echo "File tree structure created successfully."
