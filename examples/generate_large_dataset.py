"""
Generate Large Test Dataset for Prism
Creates realistic survey data with various patterns, quality issues, and edge cases
"""

import csv
import random
import math
from datetime import datetime, timedelta

# Set seed for reproducibility
random.seed(42)

def generate_normal_response(mean, sd, min_val=1, max_val=7):
    """Generate a response following normal distribution, clamped to range"""
    value = random.gauss(mean, sd)
    return max(min_val, min(max_val, round(value)))

def generate_participant_data(participant_id, participant_type):
    """Generate all survey responses for one participant"""
    row = {'id': participant_id}
    
    # PHQ-9 (Depression): 9 items, scale 0-3 -> convert to 1-4 for consistency
    if participant_type == 'depressed':
        phq_mean = 3.2  # High depression
        phq_sd = 0.8
    elif participant_type == 'anxious':
        phq_mean = 2.0  # Moderate depression
        phq_sd = 0.7
    else:
        phq_mean = 1.5  # Low depression
        phq_sd = 0.6
    
    for i in range(1, 10):
        if random.random() < 0.05:  # 5% missing
            row[f'phq{i}'] = ''
        else:
            row[f'phq{i}'] = generate_normal_response(phq_mean, phq_sd, 1, 4)
    
    # GAD-7 (Anxiety): 7 items, scale 0-3 -> convert to 1-4
    if participant_type == 'anxious':
        gad_mean = 3.5  # High anxiety
        gad_sd = 0.7
    elif participant_type == 'depressed':
        gad_mean = 2.5  # Moderate anxiety (comorbid)
        gad_sd = 0.8
    else:
        gad_mean = 1.6  # Low anxiety
        gad_sd = 0.6
    
    for i in range(1, 8):
        if random.random() < 0.05:
            row[f'gad{i}'] = ''
        else:
            row[f'gad{i}'] = generate_normal_response(gad_mean, gad_sd, 1, 4)
    
    # PSS-10 (Stress): 10 items, scale 1-5, items 4,5,7,8 reverse scored
    if participant_type == 'stressed':
        pss_mean = 4.0  # High stress
        pss_sd = 0.8
    else:
        pss_mean = 2.5  # Moderate stress
        pss_sd = 0.9
    
    for i in range(1, 11):
        if random.random() < 0.03:
            row[f'pss{i}'] = ''
        else:
            row[f'pss{i}'] = generate_normal_response(pss_mean, pss_sd, 1, 5)
    
    # PANAS Positive (10 items) and Negative (10 items): scale 1-5
    if participant_type == 'positive':
        pa_mean = 4.2
        pa_sd = 0.7
        na_mean = 1.5
        na_sd = 0.5
    elif participant_type == 'depressed' or participant_type == 'anxious':
        pa_mean = 2.0
        pa_sd = 0.8
        na_mean = 3.8
        na_sd = 0.9
    else:
        pa_mean = 3.2
        pa_sd = 0.8
        na_mean = 2.1
        na_sd = 0.7
    
    # Positive affect items
    for i in range(1, 11):
        if random.random() < 0.02:
            row[f'pa{i}'] = ''
        else:
            row[f'pa{i}'] = generate_normal_response(pa_mean, pa_sd, 1, 5)
    
    # Negative affect items
    for i in range(1, 11):
        if random.random() < 0.02:
            row[f'na{i}'] = ''
        else:
            row[f'na{i}'] = generate_normal_response(na_mean, na_sd, 1, 5)
    
    # Custom Wellbeing Scale: 15 items, scale 1-7, items 3,7,11 reverse scored
    wb_mean = 4.5 if participant_type == 'positive' else 3.0
    wb_sd = 1.2
    
    for i in range(1, 16):
        if random.random() < 0.04:
            row[f'wb{i}'] = ''
        else:
            row[f'wb{i}'] = generate_normal_response(wb_mean, wb_sd, 1, 7)
    
    # Demographics
    row['age'] = random.randint(18, 75)
    row['gender'] = random.choice(['M', 'F', 'NB', 'Other'])
    row['education'] = random.choice(['HS', 'Bachelor', 'Master', 'PhD'])
    
    return row

def add_quality_issues(row, issue_type):
    """Add specific quality issues to test detection"""
    if issue_type == 'straightline':
        # Make all PHQ items the same value
        value = random.randint(1, 4)
        for i in range(1, 10):
            row[f'phq{i}'] = value
    
    elif issue_type == 'diagonal_ascending':
        # Make GAD items follow ascending pattern
        for i in range(1, 8):
            row[f'gad{i}'] = min(4, i)
    
    elif issue_type == 'diagonal_descending':
        # Make PSS items follow descending pattern
        value = 5
        for i in range(1, 11):
            row[f'pss{i}'] = max(1, value)
            value -= 1
            if value < 1:
                value = 5
    
    elif issue_type == 'alternating':
        # Make PANAS alternate between two values
        for i in range(1, 11):
            row[f'pa{i}'] = 1 if i % 2 == 0 else 5
    
    elif issue_type == 'block_pattern':
        # First half one value, second half another
        for i in range(1, 8):
            row[f'wb{i}'] = 2
        for i in range(8, 16):
            row[f'wb{i}'] = 6
    
    elif issue_type == 'excessive_missing':
        # 80% missing data
        all_items = []
        for prefix in ['phq', 'gad', 'pss', 'pa', 'na', 'wb']:
            for i in range(1, 20):
                key = f'{prefix}{i}'
                if key in row:
                    all_items.append(key)
        
        for item in random.sample(all_items, int(len(all_items) * 0.8)):
            row[item] = ''
    
    return row

# Generate dataset
print("Generating large test dataset...")

fieldnames = ['id', 'age', 'gender', 'education']
# Add all survey items
for i in range(1, 10):
    fieldnames.append(f'phq{i}')
for i in range(1, 8):
    fieldnames.append(f'gad{i}')
for i in range(1, 11):
    fieldnames.append(f'pss{i}')
for i in range(1, 11):
    fieldnames.append(f'pa{i}')
for i in range(1, 11):
    fieldnames.append(f'na{i}')
for i in range(1, 16):
    fieldnames.append(f'wb{i}')

participants = []

# Generate different participant types
types_distribution = {
    'normal': 250,      # Healthy participants
    'depressed': 80,    # High depression
    'anxious': 70,      # High anxiety
    'stressed': 50,     # High stress
    'positive': 30,     # Very positive affect
}

participant_id = 1

# Generate normal participants
for participant_type, count in types_distribution.items():
    for _ in range(count):
        pid = f'P{participant_id:04d}'
        row = generate_participant_data(pid, participant_type)
        participants.append(row)
        participant_id += 1

# Add participants with quality issues (20 of each type)
quality_issue_types = [
    'straightline',
    'diagonal_ascending', 
    'diagonal_descending',
    'alternating',
    'block_pattern',
    'excessive_missing'
]

for issue_type in quality_issue_types:
    for _ in range(20):
        pid = f'P{participant_id:04d}'
        row = generate_participant_data(pid, 'normal')
        row = add_quality_issues(row, issue_type)
        participants.append(row)
        participant_id += 1

# Add some extreme edge cases
for _ in range(10):
    pid = f'P{participant_id:04d}'
    row = generate_participant_data(pid, 'normal')
    # All minimum values
    for key in row:
        if key not in ['id', 'age', 'gender', 'education'] and row[key] != '':
            row[key] = 1
    participants.append(row)
    participant_id += 1

for _ in range(10):
    pid = f'P{participant_id:04d}'
    row = generate_participant_data(pid, 'normal')
    # All maximum values
    for key in row:
        if key.startswith('phq') or key.startswith('gad'):
            if row[key] != '':
                row[key] = 4
        elif key not in ['id', 'age', 'gender', 'education'] and row[key] != '':
            row[key] = 7 if key.startswith('wb') else 5
    participants.append(row)
    participant_id += 1

# Shuffle for realism
random.shuffle(participants)

# Write to CSV
output_file = 'data/test_dataset_large.csv'
with open(output_file, 'w', newline='', encoding='utf-8') as f:
    writer = csv.DictWriter(f, fieldnames=fieldnames)
    writer.writeheader()
    writer.writerows(participants)

print(f"✓ Generated {len(participants)} participants")
print(f"✓ Saved to: {output_file}")
print(f"\nDataset includes:")
print(f"  - PHQ-9 (Depression): 9 items")
print(f"  - GAD-7 (Anxiety): 7 items")
print(f"  - PSS-10 (Stress): 10 items with reverse scoring")
print(f"  - PANAS: 20 items (10 positive, 10 negative)")
print(f"  - Wellbeing Scale: 15 items with reverse scoring")
print(f"  - Demographics: age, gender, education")
print(f"\nQuality issues injected:")
print(f"  - Straightlining: 20 cases")
print(f"  - Diagonal patterns: 40 cases")
print(f"  - Alternating pattern: 20 cases")
print(f"  - Block pattern: 20 cases")
print(f"  - Excessive missing: 20 cases")
print(f"  - Edge cases: 20 cases")
print(f"\nTotal items per participant: 61 survey items + 3 demographics")
