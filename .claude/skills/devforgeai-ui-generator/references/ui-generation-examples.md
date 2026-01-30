# UI Generation Examples

Complete examples of UI generation workflows for different scenarios.

---

## Example 1: Web UI from Story

**Scenario:** Generate login form from story requirements

**User input:**
```
User: "Generate UI for STORY-042 which requires a login form"
```

**Skill workflow:**

1. **Phase 1: Context Validation**
   - Validates 6 context files exist ✓
   - Loads tech-stack.md, source-tree.md, dependencies.md

2. **Phase 2: Story Analysis**
   - Reads STORY-042.story.md
   - Extracts from AC: "email input field", "password input field", "submit button"
   - Identifies: LoginForm component needed

3. **Phase 3: Interactive Discovery**
   - Asks: "What type of UI?" → User selects "Web UI"
   - Asks: "Web technology?" → User selects "React"
   - Asks: "Styling?" → User selects "Tailwind CSS"
   - Asks: "Theme?" → User selects "Dark Mode"
   - Validates: React found in tech-stack.md ✓

4. **Phase 4: Template Loading**
   - Loads: assets/web-template.jsx
   - Loads: references/web_best_practices.md
   - Loads: references/devforgeai-integration-guide.md

5. **Phase 5: Code Generation**
   - Generates LoginForm.jsx with:
     - Email input with validation
     - Password input with show/hide toggle
     - Submit button with loading state
     - Error message display
     - Tailwind CSS classes
     - Dark mode color scheme
     - ARIA labels and roles
   - Saves to: devforgeai/specs/ui/LoginForm.jsx

6. **Phase 6: Documentation**
   - Creates UI-SPEC-SUMMARY.md
   - Updates STORY-042.story.md with UI component reference

7. **Phase 6 Step 3.5: Formatter**
   - Invokes ui-spec-formatter subagent
   - Formatter returns SUCCESS status with formatted display

8. **Phase 7: Validation**
   - Checks all 10 required sections present ✓
   - Scans for placeholders: None found ✓
   - Validates against context files: All passed ✓
   - Status: SUCCESS

**Output:**
- LoginForm.jsx in devforgeai/specs/ui/
- UI-SPEC-SUMMARY.md created
- STORY-042 updated
- Ready for /dev STORY-042

---

## Example 2: Native GUI without Story

**Scenario:** Create settings dialog for desktop application

**User input:**
```
User: "Create a settings dialog for the application"
```

**Skill workflow:**

1. **Phase 1: Context Validation**
   - Validates context files ✓

2. **Phase 2: Story Analysis**
   - No story provided, skip to Phase 3

3. **Phase 3: Interactive Discovery**
   - Asks: "What type of UI?" → User selects "Native GUI"
   - Asks: "GUI technology?" → User selects "C# WPF"
   - Asks: "Components?" → User describes "Tabs for General, Advanced, About"
   - Validates: WPF found in tech-stack.md ✓

4. **Phase 4: Template Loading**
   - Loads: assets/gui-template.wpf.xaml
   - Loads: references/gui_best_practices.md

5. **Phase 5: Code Generation**
   - Generates SettingsDialog.xaml with:
     - Window with TabControl
     - GeneralTab with common settings
     - AdvancedTab with technical options
     - AboutTab with version info
     - Proper XAML namespaces
     - MVVM-friendly structure

6. **Phase 6: Documentation**
   - Creates UI-SPEC-SUMMARY.md
   - No story to update (standalone mode)

7. **Phase 6 Step 3.5: Formatter**
   - Invokes ui-spec-formatter
   - Returns SUCCESS with formatted display

8. **Phase 7: Validation**
   - All validations pass ✓

**Output:**
- SettingsDialog.xaml in devforgeai/specs/ui/
- UI-SPEC-SUMMARY.md created
- Ready for implementation

---

## Example 3: Terminal UI for Data Display

**Scenario:** Display server status in terminal table

**User input:**
```
User: "Generate a terminal table to display server status"
```

**Skill workflow:**

1. **Phase 1: Context Validation**
   - Validates context files ✓

2. **Phase 2: Story Analysis**
   - No story provided, skip

3. **Phase 3: Interactive Discovery**
   - Asks: "What type of UI?" → User selects "Terminal UI"
   - Asks: "Formatting style?" → User selects "Box Drawing"
   - Asks: "Components?" → User describes "Status table: server name, uptime, CPU, memory"

4. **Phase 4: Template Loading**
   - Loads: assets/tui-template.py
   - Loads: references/tui_best_practices.md

5. **Phase 5: Code Generation**
   - Generates format_server_status.py with:
     - Box-drawing characters for table borders
     - Column alignment
     - Color coding (green=healthy, yellow=warning, red=error)
     - Header row with labels
     - Data row formatting

6. **Phase 6: Documentation**
   - Creates UI-SPEC-SUMMARY.md

7. **Phase 6 Step 3.5: Formatter**
   - Invokes formatter
   - Returns SUCCESS

8. **Phase 7: Validation**
   - All validations pass ✓

**Output:**
- format_server_status.py in devforgeai/specs/ui/
- Ready to use in terminal application

---

## Example 4: Handling Technology Conflict

**Scenario:** User requests unapproved technology

**User input:**
```
User: "Generate Vue.js component for user profile"
(Assume tech-stack.md specifies React, not Vue)
```

**Skill workflow:**

1. **Phases 1-2:** Normal execution

2. **Phase 3: Interactive Discovery**
   - User selects "Web UI" → "Vue.js"

3. **Step 3.3: Validate Against Context**
   - Reads tech-stack.md
   - Detects: User selected "Vue.js", but tech-stack.md specifies "React"
   - **CONFLICT DETECTED**

4. **Conflict Resolution:**
   ```
   AskUserQuestion:
     Question: "You selected Vue.js, but tech-stack.md specifies React. Which should be used?"
     Header: "Tech Conflict"
     Options: [
       "Use Vue.js (update tech-stack.md and create ADR)",
       "Use React (follow existing standard)"
     ]
   ```

5. **User selects "Use React":**
   - Override user's original selection
   - Proceed with React
   - Continue to Phase 4 with React template

6. **Phases 4-7:** Generate React component instead of Vue

**Alternative: User selects "Use Vue.js":**
- Display: "This requires updating tech-stack.md and creating ADR"
- Ask: "Update tech-stack.md now or manually?"
- If update now: Edit tech-stack.md, create ADR stub
- Proceed with Vue.js
- Note in documentation: "tech-stack.md updated per user request"

---

## Example 5: Incomplete Specification (Phase 7 Resolution)

**Scenario:** Generated spec missing accessibility section

**Workflow:**

1. **Phases 1-6:** UI generated, documentation created

2. **Phase 7 Step 7.1: Completeness Check**
   - Reads generated spec
   - Missing: Accessibility section
   - Status: INCOMPLETE

3. **Step 7.4: User Resolution**
   ```
   AskUserQuestion:
     Question: "The UI specification is missing 1 required section: Accessibility. How should I proceed?"
     Header: "Incomplete Spec"
     Options: [
       "Provide missing information",
       "Use framework defaults",
       "Accept as-is",
       "Regenerate specification"
     ]
   ```

4. **User selects "Use framework defaults":**
   - Display defaults:
     ```
     DevForgeAI Standard Defaults:
     - Accessibility: WCAG 2.1 AA
     - Features: keyboard navigation, screen reader support, focus indicators
     ```
   - Ask: "Apply these defaults?"
   - User confirms: "Yes"

5. **Apply defaults:**
   ```
   Edit(file_path="devforgeai/specs/ui/Component.jsx",
        old_string="## Integration Points",
        new_string="## Accessibility\n- Level: WCAG 2.1 AA\n- Keyboard navigation: Yes\n- Screen reader: ARIA labels\n- Focus management: Visible focus indicators\n\n## Integration Points")
   ```

6. **Re-validate:**
   - Step 7.1 passes: All 10 sections present ✓
   - Continue to Steps 7.2-7.5

**Output:**
- Specification complete
- User approved defaults applied
- Validation passes
