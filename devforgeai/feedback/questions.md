# Feedback Question Bank

Structure and usage documentation for retrospective questions.

---

## Organization

Questions are organized by workflow type and outcome status:
- **Workflow types:** dev, qa, orchestrate, release, ideate, create-story, create-epic, create-sprint
- **Outcome status:** success, failed, partial

---

## Success Questions

### Development Workflow (dev)

**dev_success_01:** How confident are you in the TDD workflow? (rating 1-5)
- 1 = Not confident at all
- 3 = Somewhat confident
- 5 = Very confident

**dev_success_02:** Which phase was most challenging? (multiple choice)
- Red (writing failing tests)
- Green (implementing code to pass)
- Refactor (improving code quality)
- Integration (combining components)

**dev_success_03:** Were test coverage requirements clear? (rating 1-5)
- 1 = Very unclear
- 3 = Neutral
- 5 = Very clear

**dev_success_04:** Would you use this workflow again? (multiple choice)
- Yes, definitely
- Maybe, with improvements
- No, prefer another approach

**dev_success_05:** Additional feedback on development experience (open text, optional)
- Min 10 characters, max 5,000 characters

---

### QA Workflow (qa)

**qa_success_01:** Were coverage requirements clear? (rating 1-5)
- 1 = Very unclear
- 5 = Very clear

**qa_success_02:** Was violation guidance helpful? (rating 1-5)
- 1 = Not helpful
- 5 = Very helpful

**qa_success_03:** How long did it take to prepare for release? (open text)
- Share your experience with QA validation time

**qa_success_04:** Overall QA experience rating (rating 1-5)
- 1 = Poor
- 5 = Excellent

---

### Orchestrate Workflow (orchestrate)

**orchestrate_success_01:** How confident are you in the full lifecycle workflow? (rating 1-5)
- 1 = Not confident
- 5 = Very confident

**orchestrate_success_02:** Were phase transitions smooth? (multiple choice)
- Yes, seamless
- Mostly smooth, minor issues
- No, had significant problems

**orchestrate_success_03:** Overall orchestration experience (open text)
- What worked well? What could improve?

---

### Release Workflow (release)

**release_success_01:** Were deployment steps clear? (rating 1-5)
- 1 = Very unclear
- 5 = Very clear

**release_success_02:** Deployment confidence level (rating 1-5)
- 1 = Not confident
- 5 = Very confident

**release_success_03:** Post-deployment verification sufficient? (multiple choice)
- Yes, comprehensive
- Adequate
- No, needed more checks

---

### Ideation Workflow (ideate)

**ideate_success_01:** Did the discovery process surface all requirements? (rating 1-5)
- 1 = Missed many
- 5 = Captured everything

**ideate_success_02:** Were questions helpful in clarifying the idea? (rating 1-5)
- 1 = Not helpful
- 5 = Very helpful

**ideate_success_03:** What would improve the ideation process? (open text)

---

### Story Creation Workflow (create-story)

**story_success_01:** Are acceptance criteria clear and testable? (rating 1-5)
- 1 = Very unclear
- 5 = Very clear

**story_success_02:** Technical specification completeness (rating 1-5)
- 1 = Incomplete
- 5 = Comprehensive

**story_success_03:** What would improve story creation? (open text)

---

## Failure Questions

### Development Failure (dev)

**dev_failure_01:** What blocked completion? (open text)
- Describe the main obstacle encountered

**dev_failure_02:** Was it a test framework issue? (multiple choice)
- Yes, test framework problem
- Yes, testing approach unclear
- No, implementation issue
- No, requirements unclear

**dev_failure_03:** What would help overcome this blocker? (open text)
- Specific guidance, examples, or tools needed

**dev_failure_04:** Was error guidance helpful? (rating 1-5)
- 1 = Not helpful
- 5 = Very helpful

---

### QA Failure (qa)

**qa_failure_01:** Coverage not met - which area? (open text)
- Business logic, application, infrastructure?

**qa_failure_02:** Were violation details helpful? (multiple choice)
- Yes, very clear
- Somewhat helpful
- No, too vague

**qa_failure_03:** Is the path forward clear? (multiple choice)
- Yes, I know what to do
- Partially clear
- No, need more guidance

**qa_failure_04:** What would help pass QA? (open text)
- Specific improvements needed

---

### Orchestrate Failure (orchestrate)

**orchestrate_failure_01:** Which phase failed? (multiple choice)
- Development
- QA validation
- Staging release
- Production release

**orchestrate_failure_02:** Was checkpoint recovery helpful? (rating 1-5)
- 1 = Not helpful
- 5 = Very helpful

**orchestrate_failure_03:** What would prevent this failure? (open text)
- Suggestions for improvement

---

### Release Failure (release)

**release_failure_01:** What caused the deployment failure? (open text)
- Infrastructure, configuration, tests?

**release_failure_02:** Was rollback procedure clear? (multiple choice)
- Yes, straightforward
- Somewhat clear
- No, unclear

**release_failure_03:** What would improve release reliability? (open text)

---

## Partial Success Questions

### Development Partial (dev)

**dev_partial_01:** Which parts succeeded? (multiple choice, multi-select)
- Tests written
- Tests passing
- Code implemented
- Code refactored

**dev_partial_02:** What remains incomplete? (open text)

**dev_partial_03:** Can you complete the rest independently? (multiple choice)
- Yes, I know what to do
- Need guidance
- Need significant help

---

### QA Partial (qa)

**qa_partial_01:** Which checks passed? (multiple choice, multi-select)
- Build succeeded
- Tests passed
- Coverage met
- No violations

**qa_partial_02:** What prevented full success? (open text)

**qa_partial_03:** Time needed to address remaining issues? (open text)

---

## Response Types

### Rating (1-5 Likert Scale)
- **1:** Strongly disagree / Very poor
- **2:** Disagree / Below average
- **3:** Neutral / Average
- **4:** Agree / Above average
- **5:** Strongly agree / Excellent

### Multiple Choice
- Select one option from provided list
- Options vary by question

### Open Text
- Free-form text response
- **Minimum:** 10 characters (prevent spam)
- **Maximum:** 5,000 characters (reasonable limit)
- Optional unless marked required

---

## Implementation

See `questions.yaml` for complete machine-readable implementation of all questions with:
- Question IDs
- Display text
- Response types
- Validation rules
- Conditional logic (which questions to ask when)

---

## Adding New Questions

When adding questions:
1. Follow naming convention: `{workflow}_{status}_{number}`
2. Add to questions.yaml
3. Update this documentation
4. Test with sample feedback session
5. Review with users for clarity

---

## Related Documentation

- JSON Schema: `devforgeai/feedback/schema.json`
- User Guide: `devforgeai/feedback/USER-GUIDE.md`
- Maintainer Guide: `devforgeai/feedback/MAINTAINER-GUIDE.md`
