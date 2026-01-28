# Adaptive Questioning Engine - Configuration Parameters

## Adjustable Parameters

### 1. Base Question Counts

Location: `AdaptiveQuestioningEngine.BASE_QUESTION_COUNTS`

```python
BASE_QUESTION_COUNTS = {
    ('dev', 'passed'): 6,
    ('dev', 'failed'): 8,
    ('qa', 'passed'): 6,
    ('qa', 'failed'): 8,
    ('release', 'passed'): 6,
    ('release', 'failed'): 7,
    ('orchestrate', 'passed'): 6,
    ('orchestrate', 'failed'): 7,
    # Default: 6 if combination not found
}
```

**Purpose:** Set baseline question count for each (operation_type, success_status) pair

**Typical Range:** 5-8 questions

---

### 2. Modifier Amounts

Location: `AdaptiveQuestioningEngine.select_questions()` method

| Modifier | Current Value | Purpose | Range |
|----------|---------------|---------|-------|
| Error context | +2 | Add investigation questions when errors occur | +1 to +3 |
| First-time user | +2 | Provide more context for new operation types | +2 to +4 |
| Repeat user | ×0.7, min 4 | Reduce for experienced users | ×0.5 to ×0.9 |
| Rapid mode | -3 | Reduce for fast-paced operations | -2 to -4 |
| Performance outlier | +1 | Add performance investigation question | +1 to +2 |

---

### 3. Bounds

| Bound | Value | Purpose |
|-------|-------|---------|
| MIN_QUESTIONS | 2 | Emergency fallback (never go below) |
| MAX_QUESTIONS | 10 | User patience limit (never exceed) |

---

### 4. Deduplication Window

**Value:** 30 days

**Purpose:** Skip questions answered within this timeframe (except priority 1)

**Location:** `AdaptiveQuestioningEngine._is_question_duplicate()`

**Adjustable Range:** 7-90 days

---

### 5. Rapid Mode Detection

**Threshold:** 3+ operations within 10 minutes

**Location:** `AdaptiveQuestioningEngine._detect_rapid_mode()`

**Adjustable:** Change `operations_count_threshold` (default 3) or `time_window_minutes` (default 10)

---

### 6. Repeat User Threshold

**Threshold:** 3+ operations of same type

**Location:** `AdaptiveQuestioningEngine.select_questions()` - `is_repeat_user` detection

**Adjustable:** Change `>= 3` to different value (e.g., `>= 5` for stricter repeat detection)

---

## Configuration Example

```python
# Create engine with custom question bank
question_bank = load_yaml("custom-questions.yaml")
engine = AdaptiveQuestioningEngine(question_bank)

# Adjust base counts (requires code modification)
engine.BASE_QUESTION_COUNTS[('dev', 'passed')] = 8  # Increase dev questions

# Use engine
result = engine.select_questions(context)
```

---

## Performance Tuning

**If selection too slow:**
- Reduce question bank size (<100 questions per operation type)
- Simplify deduplication logic
- Cache question sets

**If question counts too high:**
- Reduce base counts (5 instead of 6)
- Increase repeat user multiplier (×0.5 instead of ×0.7)
- Lower MAX_QUESTIONS to 8

**If question counts too low:**
- Increase base counts (7 instead of 6)
- Increase first-time modifier (+3 instead of +2)
- Raise MIN_QUESTIONS to 3
