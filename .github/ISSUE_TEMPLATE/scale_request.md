---
name: Scale Library Request
about: Request a pre-built scale configuration
title: "[SCALE] "
labels: scale-library, enhancement
assignees: ""
---

## 📚 Scale Library Request

### **Scale Information**

**Scale Name:**
[e.g., Patient Health Questionnaire-9 (PHQ-9)]

**Abbreviation:**
[e.g., PHQ-9]

**Number of Items:**
[e.g., 9]

**Response Scale:**
[e.g., 0 = Not at all, 1 = Several days, 2 = More than half the days, 3 = Nearly every day]

**Scoring Method:**

- [ ] Sum score
- [ ] Mean score
- [ ] Weighted score
- [ ] Other: ****\_\_\_****

**Reverse-Scored Items:**
[e.g., Items 3, 5, 7 are reverse-scored]

### **Citation**

Provide the original reference for this scale:

```
Author, A. B., & Author, C. D. (Year). Title of scale. Journal Name, Volume(Issue), pages.
```

### **Public Domain / Permission**

- [ ] This scale is in the public domain
- [ ] I have permission to share this scale
- [ ] This is a widely-used published scale
- [ ] Other: ****\_\_\_****

### **Use Case**

How would this scale be useful to the research community?

### **Sample Configuration (Optional)**

If you have a working config, paste it here:

```toml
[survey]
name = "Your Scale Name"
min_score = 0
max_score = 3

[scales.total_score]
name = "Total Score"
method = "sum"
items = [1, 2, 3, 4, 5, 6, 7, 8, 9]
reverse = []
```

### **Additional Notes**

Any special scoring rules, subscales, or clinical cutoffs?
