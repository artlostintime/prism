# R Script for Clinical Interactions & Trainee Well-Being
# Generated: 2026-01-03 13:13:11

# Load required packages
library(tidyverse)

# Import data
data <- read_csv(".\examples\output_clean.csv")

# View structure
glimpse(data)

# Summary statistics for scales
data %>% select(emotional_exhaustion_mean) %>% summary()
data %>% select(alliance_total_mean) %>% summary()
data %>% select(supervision_rapport_mean) %>% summary()
data %>% select(depersonalization_mean) %>% summary()
data %>% select(peer_support_mean) %>% summary()

# Filter clean records only
clean_data <- data %>% filter(quality_flag == 'OK')
message(sprintf('Clean records: %d / %d (%.1f%%)', nrow(clean_data), nrow(data), 100 * nrow(clean_data) / nrow(data)))
