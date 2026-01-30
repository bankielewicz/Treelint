# Phase 4: Template & Best Practices Loading

Load appropriate templates and best practices based on user selections.

**Objective:** Load framework-specific templates and best practices based on user's technology choices.

---

## Steps

### Step 4.1: Determine Template Files

**Map user's technology choice to template file:**

**Web Technologies:**
- React → `assets/web-template.jsx`
- Blazor Server → `assets/web-template.blazor.razor`
- Blazor WASM → `assets/web-template.blazor.razor`
- ASP.NET MVC → `assets/web-template.aspnet.cshtml`
- Plain HTML → `assets/web-template.html`

**Native GUI Technologies:**
- Python Tkinter → `assets/gui-template.py`
- C# WPF → `assets/gui-template.wpf.xaml`
- C# .NET MAUI → `assets/gui-template.wpf.xaml` (similar structure)
- Python PyQt → `assets/gui-template.py` (adapt for PyQt)

**Terminal UI:**
- All terminal options → `assets/tui-template.py`

---

### Step 4.2: Load Template

**Use Read tool:**
```
Read(file_path=".claude/skills/devforgeai-ui-generator/assets/${template_file}")
```

**Template provides:**
- Boilerplate structure for chosen technology
- Import/namespace declarations
- Component skeleton
- Placeholder sections for customization

---

### Step 4.3: Load Best Practices

**Use Read tool based on UI type:**

**For Web UI:**
```
Read(file_path=".claude/skills/devforgeai-ui-generator/references/web_best_practices.md")
```

**For Native GUI:**
```
Read(file_path=".claude/skills/devforgeai-ui-generator/references/gui_best_practices.md")
```

**For Terminal UI:**
```
Read(file_path=".claude/skills/devforgeai-ui-generator/references/tui_best_practices.md")
```

**Best practices cover:**
- Technology-specific patterns
- Accessibility guidelines
- Layout organization
- Naming conventions
- Platform-specific considerations

---

### Step 4.4: Load Integration Guide

**Always load the DevForgeAI integration guide:**
```
Read(file_path=".claude/skills/devforgeai-ui-generator/references/devforgeai-integration-guide.md")
```

**Integration guide provides:**
- Context file integration patterns
- How to respect tech-stack.md, source-tree.md, dependencies.md
- Common integration scenarios
- Framework-specific considerations

---

## Output

**Phase 4 produces:**
- Templates loaded into context
- Best practices loaded into context
- Integration guide loaded into context
- Ready to proceed to Phase 5 (Code Generation)

**Token usage:**
- Template: ~500-1,500 tokens (depending on complexity)
- Best practices: ~2,000-3,000 tokens
- Integration guide: ~2,000 tokens
- **Total: ~4,000-6,500 tokens**
