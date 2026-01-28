# Native GUI Best Practices

This guide provides best practices for generating native desktop GUI applications using WPF (C#), Tkinter (Python), .NET MAUI, and other GUI frameworks.

---

## Layout Organization

Organize UI elements logically to create intuitive, maintainable interfaces.

### Logical Grouping

Group related controls together using container elements.

#### WPF (XAML)

```xml
<!-- Group related inputs in a StackPanel or Grid -->
<StackPanel>
    <!-- Personal Information Group -->
    <GroupBox Header="Personal Information" Margin="0,0,0,10">
        <Grid>
            <Grid.RowDefinitions>
                <RowDefinition Height="Auto"/>
                <RowDefinition Height="Auto"/>
            </Grid.RowDefinitions>
            <Grid.ColumnDefinitions>
                <ColumnDefinition Width="Auto"/>
                <ColumnDefinition Width="*"/>
            </Grid.ColumnDefinitions>

            <Label Grid.Row="0" Grid.Column="0" Content="Name:" Margin="5"/>
            <TextBox Grid.Row="0" Grid.Column="1" x:Name="NameTextBox" Margin="5"/>

            <Label Grid.Row="1" Grid.Column="0" Content="Email:" Margin="5"/>
            <TextBox Grid.Row="1" Grid.Column="1" x:Name="EmailTextBox" Margin="5"/>
        </Grid>
    </GroupBox>

    <!-- Account Settings Group -->
    <GroupBox Header="Account Settings" Margin="0,0,0,10">
        <StackPanel>
            <CheckBox Content="Receive notifications" Margin="5"/>
            <CheckBox Content="Enable dark mode" Margin="5"/>
        </StackPanel>
    </GroupBox>
</StackPanel>
```

#### Python Tkinter

```python
import tkinter as tk
from tkinter import ttk

root = tk.Tk()
root.title("User Profile")

# Personal Information Frame
personal_frame = ttk.LabelFrame(root, text="Personal Information", padding=10)
personal_frame.grid(row=0, column=0, padx=10, pady=10, sticky="ew")

ttk.Label(personal_frame, text="Name:").grid(row=0, column=0, sticky="w", pady=5)
name_entry = ttk.Entry(personal_frame, width=30)
name_entry.grid(row=0, column=1, pady=5)

ttk.Label(personal_frame, text="Email:").grid(row=1, column=0, sticky="w", pady=5)
email_entry = ttk.Entry(personal_frame, width=30)
email_entry.grid(row=1, column=1, pady=5)

# Account Settings Frame
settings_frame = ttk.LabelFrame(root, text="Account Settings", padding=10)
settings_frame.grid(row=1, column=0, padx=10, pady=10, sticky="ew")

notifications_var = tk.BooleanVar()
ttk.Checkbutton(settings_frame, text="Receive notifications", variable=notifications_var).pack(anchor="w")

dark_mode_var = tk.BooleanVar()
ttk.Checkbutton(settings_frame, text="Enable dark mode", variable=dark_mode_var).pack(anchor="w")
```

---

## Component Naming Conventions

Use clear, consistent naming for UI components.

### WPF Naming

```xml
<!-- Pattern: [Purpose][ControlType] -->
<Window>
    <!-- Buttons -->
    <Button x:Name="SubmitButton" Content="Submit"/>
    <Button x:Name="CancelButton" Content="Cancel"/>

    <!-- Text Inputs -->
    <TextBox x:Name="UsernameTextBox"/>
    <PasswordBox x:Name="PasswordBox"/>

    <!-- Lists -->
    <ListBox x:Name="UsersListBox"/>
    <ComboBox x:Name="CountryComboBox"/>

    <!-- Labels -->
    <Label x:Name="StatusLabel" Content="Ready"/>
    <TextBlock x:Name="ErrorMessageTextBlock" Foreground="Red"/>
</Window>
```

### Python Tkinter Naming

```python
# Pattern: purpose_control_type (lowercase with underscores)

# Buttons
submit_button = ttk.Button(root, text="Submit")
cancel_button = ttk.Button(root, text="Cancel")

# Text Inputs
username_entry = ttk.Entry(root)
password_entry = ttk.Entry(root, show="*")

# Lists
users_listbox = tk.Listbox(root)
country_combobox = ttk.Combobox(root)

# Labels
status_label = ttk.Label(root, text="Ready")
error_message_label = ttk.Label(root, foreground="red")
```

---

## Tab Order and Keyboard Navigation

Ensure logical tab order for keyboard-only users.

### WPF Tab Order

```xml
<!-- Use TabIndex to control tab order -->
<Grid>
    <TextBox x:Name="FirstNameTextBox" TabIndex="0"/>
    <TextBox x:Name="LastNameTextBox" TabIndex="1"/>
    <TextBox x:Name="EmailTextBox" TabIndex="2"/>
    <Button Content="Submit" TabIndex="3"/>
    <Button Content="Cancel" TabIndex="4"/>
</Grid>

<!-- Disable tab stop for read-only elements -->
<Label Content="Instructions" IsTabStop="False"/>
```

### Python Tkinter Tab Order

```python
# Tkinter follows creation order by default
# Create widgets in logical tab order

first_name_entry = ttk.Entry(root)
first_name_entry.grid(row=0, column=1)

last_name_entry = ttk.Entry(root)
last_name_entry.grid(row=1, column=1)

email_entry = ttk.Entry(root)
email_entry.grid(row=2, column=1)

# Set initial focus
first_name_entry.focus()
```

### Keyboard Shortcuts

#### WPF

```xml
<!-- Use access keys (underscored letters) -->
<Button Content="_Submit" Click="SubmitButton_Click"/>
<!-- Alt+S activates this button -->

<Label Content="_Username:" Target="{Binding ElementName=UsernameTextBox}"/>
<!-- Alt+U focuses the username textbox -->

<!-- Use command bindings for common operations -->
<Window.InputBindings>
    <KeyBinding Key="S" Modifiers="Ctrl" Command="{Binding SaveCommand}"/>
    <KeyBinding Key="O" Modifiers="Ctrl" Command="{Binding OpenCommand}"/>
</Window.InputBindings>
```

#### Python Tkinter

```python
# Bind keyboard shortcuts
root.bind('<Control-s>', lambda e: save_file())
root.bind('<Control-o>', lambda e: open_file())

# Add accelerator keys to menu items
menubar = tk.Menu(root)
file_menu = tk.Menu(menubar, tearoff=0)
file_menu.add_command(label="Save", command=save_file, accelerator="Ctrl+S")
file_menu.add_command(label="Open", command=open_file, accelerator="Ctrl+O")
```

---

## Layout Patterns

### Grid Layout (Recommended for Forms)

#### WPF Grid

```xml
<Grid>
    <!-- Define rows and columns -->
    <Grid.RowDefinitions>
        <RowDefinition Height="Auto"/>
        <RowDefinition Height="Auto"/>
        <RowDefinition Height="Auto"/>
        <RowDefinition Height="*"/>
        <RowDefinition Height="Auto"/>
    </Grid.RowDefinitions>
    <Grid.ColumnDefinitions>
        <ColumnDefinition Width="Auto"/>
        <ColumnDefinition Width="*"/>
    </Grid.ColumnDefinitions>

    <!-- Place elements in grid cells -->
    <Label Grid.Row="0" Grid.Column="0" Content="Username:"/>
    <TextBox Grid.Row="0" Grid.Column="1" x:Name="UsernameTextBox"/>

    <Label Grid.Row="1" Grid.Column="0" Content="Password:"/>
    <PasswordBox Grid.Row="1" Grid.Column="1" x:Name="PasswordBox"/>

    <Button Grid.Row="2" Grid.Column="1" Content="Login" HorizontalAlignment="Left"/>
</Grid>
```

#### Python Tkinter Grid

```python
# Configure grid weights for responsive layout
root.columnconfigure(1, weight=1)

# Place widgets using grid
ttk.Label(root, text="Username:").grid(row=0, column=0, sticky="w", padx=5, pady=5)
username_entry = ttk.Entry(root)
username_entry.grid(row=0, column=1, sticky="ew", padx=5, pady=5)

ttk.Label(root, text="Password:").grid(row=1, column=0, sticky="w", padx=5, pady=5)
password_entry = ttk.Entry(root, show="*")
password_entry.grid(row=1, column=1, sticky="ew", padx=5, pady=5)

login_button = ttk.Button(root, text="Login")
login_button.grid(row=2, column=1, sticky="w", padx=5, pady=10)
```

### Stack Layout (For Simple Vertical/Horizontal Lists)

#### WPF StackPanel

```xml
<!-- Vertical stack (default) -->
<StackPanel Orientation="Vertical">
    <TextBlock Text="Select an option:"/>
    <RadioButton Content="Option 1" GroupName="Options"/>
    <RadioButton Content="Option 2" GroupName="Options"/>
    <RadioButton Content="Option 3" GroupName="Options"/>
</StackPanel>

<!-- Horizontal stack -->
<StackPanel Orientation="Horizontal">
    <Button Content="OK" Margin="0,0,5,0"/>
    <Button Content="Cancel" Margin="0,0,5,0"/>
    <Button Content="Apply"/>
</StackPanel>
```

#### Python Tkinter Pack

```python
# Vertical packing (default)
frame = ttk.Frame(root)
frame.pack(fill="both", expand=True, padx=10, pady=10)

ttk.Label(frame, text="Select an option:").pack(anchor="w")

option_var = tk.StringVar(value="option1")
ttk.Radiobutton(frame, text="Option 1", variable=option_var, value="option1").pack(anchor="w")
ttk.Radiobutton(frame, text="Option 2", variable=option_var, value="option2").pack(anchor="w")
ttk.Radiobutton(frame, text="Option 3", variable=option_var, value="option3").pack(anchor="w")

# Horizontal packing
button_frame = ttk.Frame(root)
button_frame.pack(fill="x", padx=10, pady=10)

ttk.Button(button_frame, text="OK").pack(side="left", padx=5)
ttk.Button(button_frame, text="Cancel").pack(side="left", padx=5)
ttk.Button(button_frame, text="Apply").pack(side="left", padx=5)
```

---

## Data Binding (WPF)

Use data binding to separate UI from data logic.

### MVVM Pattern

```xml
<!-- View (XAML) -->
<Window x:Class="App.Views.LoginWindow"
        xmlns:vm="clr-namespace:App.ViewModels">
    <Window.DataContext>
        <vm:LoginViewModel/>
    </Window.DataContext>

    <Grid>
        <TextBox Text="{Binding Username, UpdateSourceTrigger=PropertyChanged}"/>
        <PasswordBox x:Name="PasswordBox"/>
        <Button Content="Login" Command="{Binding LoginCommand}"
                CommandParameter="{Binding ElementName=PasswordBox}"/>
        <TextBlock Text="{Binding ErrorMessage}" Foreground="Red"
                   Visibility="{Binding HasError, Converter={StaticResource BoolToVisibilityConverter}}"/>
    </Grid>
</Window>
```

```csharp
// ViewModel (C#)
public class LoginViewModel : INotifyPropertyChanged
{
    private string _username;
    private string _errorMessage;
    private bool _hasError;

    public string Username
    {
        get => _username;
        set
        {
            _username = value;
            OnPropertyChanged(nameof(Username));
        }
    }

    public string ErrorMessage
    {
        get => _errorMessage;
        set
        {
            _errorMessage = value;
            OnPropertyChanged(nameof(ErrorMessage));
        }
    }

    public bool HasError
    {
        get => _hasError;
        set
        {
            _hasError = value;
            OnPropertyChanged(nameof(HasError));
        }
    }

    public ICommand LoginCommand { get; }

    public LoginViewModel()
    {
        LoginCommand = new RelayCommand(ExecuteLogin, CanExecuteLogin);
    }

    private bool CanExecuteLogin(object parameter)
    {
        return !string.IsNullOrWhiteSpace(Username);
    }

    private async void ExecuteLogin(object parameter)
    {
        var passwordBox = parameter as PasswordBox;
        var password = passwordBox?.Password;

        try
        {
            await AuthService.LoginAsync(Username, password);
            // Navigate to main window
        }
        catch (Exception ex)
        {
            ErrorMessage = ex.Message;
            HasError = true;
        }
    }

    public event PropertyChangedEventHandler PropertyChanged;

    protected void OnPropertyChanged(string propertyName)
    {
        PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(propertyName));
    }
}
```

---

## Validation

### WPF Validation

```xml
<!-- IDataErrorInfo validation -->
<TextBox Text="{Binding Email, ValidatesOnDataErrors=True, UpdateSourceTrigger=PropertyChanged}">
    <TextBox.Style>
        <Style TargetType="TextBox">
            <Style.Triggers>
                <Trigger Property="Validation.HasError" Value="True">
                    <Setter Property="ToolTip" Value="{Binding RelativeSource={RelativeSource Self}, Path=(Validation.Errors)[0].ErrorContent}"/>
                    <Setter Property="BorderBrush" Value="Red"/>
                </Trigger>
            </Style.Triggers>
        </Style>
    </TextBox.Style>
</TextBox>
```

```csharp
// ViewModel implementing IDataErrorInfo
public class UserViewModel : INotifyPropertyChanged, IDataErrorInfo
{
    private string _email;

    public string Email
    {
        get => _email;
        set
        {
            _email = value;
            OnPropertyChanged(nameof(Email));
        }
    }

    public string Error => null;

    public string this[string columnName]
    {
        get
        {
            string result = null;
            switch (columnName)
            {
                case nameof(Email):
                    if (string.IsNullOrWhiteSpace(Email))
                        result = "Email is required";
                    else if (!IsValidEmail(Email))
                        result = "Email format is invalid";
                    break;
            }
            return result;
        }
    }

    private bool IsValidEmail(string email)
    {
        return System.Text.RegularExpressions.Regex.IsMatch(email,
            @"^[^@\s]+@[^@\s]+\.[^@\s]+$");
    }
}
```

### Python Tkinter Validation

```python
def validate_email(email):
    """Validate email format"""
    import re
    pattern = r'^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$'
    return re.match(pattern, email) is not None

def on_submit():
    """Validate form before submission"""
    email = email_entry.get()

    # Clear previous errors
    error_label.config(text="")

    # Validate
    if not email:
        error_label.config(text="Email is required", foreground="red")
        email_entry.focus()
        return

    if not validate_email(email):
        error_label.config(text="Invalid email format", foreground="red")
        email_entry.focus()
        return

    # Submit form
    print(f"Submitting: {email}")

# Entry widget with validation
email_entry = ttk.Entry(root, width=30)
email_entry.grid(row=0, column=1, padx=5, pady=5)

# Error label
error_label = ttk.Label(root, text="", foreground="red")
error_label.grid(row=1, column=0, columnspan=2, padx=5, pady=5)

# Submit button
submit_button = ttk.Button(root, text="Submit", command=on_submit)
submit_button.grid(row=2, column=1, padx=5, pady=10)
```

---

## Dialog Boxes

### WPF MessageBox

```csharp
// Confirmation dialog
MessageBoxResult result = MessageBox.Show(
    "Are you sure you want to delete this item?",
    "Confirm Delete",
    MessageBoxButton.YesNo,
    MessageBoxImage.Question
);

if (result == MessageBoxResult.Yes)
{
    // Delete item
}

// Error dialog
MessageBox.Show(
    "An error occurred while saving the file.",
    "Error",
    MessageBoxButton.OK,
    MessageBoxImage.Error
);
```

### Python Tkinter MessageBox

```python
from tkinter import messagebox

# Confirmation dialog
result = messagebox.askyesno(
    "Confirm Delete",
    "Are you sure you want to delete this item?"
)

if result:
    # Delete item
    pass

# Error dialog
messagebox.showerror(
    "Error",
    "An error occurred while saving the file."
)

# Info dialog
messagebox.showinfo(
    "Success",
    "File saved successfully."
)
```

---

## Threading and Async Operations

### WPF Async/Await

```csharp
private async void LoadDataButton_Click(object sender, RoutedEventArgs e)
{
    // Disable button during operation
    LoadDataButton.IsEnabled = false;
    StatusLabel.Content = "Loading...";

    try
    {
        // Run async operation
        var data = await LoadDataAsync();

        // Update UI on UI thread
        DataGrid.ItemsSource = data;
        StatusLabel.Content = "Data loaded successfully";
    }
    catch (Exception ex)
    {
        MessageBox.Show($"Error loading data: {ex.Message}", "Error", MessageBoxButton.OK, MessageBoxImage.Error);
        StatusLabel.Content = "Error loading data";
    }
    finally
    {
        LoadDataButton.IsEnabled = true;
    }
}

private async Task<List<DataItem>> LoadDataAsync()
{
    // Simulate long-running operation
    await Task.Delay(2000);
    return await _dataService.GetDataAsync();
}
```

### Python Tkinter Threading

```python
import threading

def load_data():
    """Long-running operation in background thread"""
    # Disable button
    load_button.config(state="disabled")
    status_label.config(text="Loading...")

    def background_task():
        try:
            # Simulate long-running operation
            import time
            time.sleep(2)
            data = fetch_data_from_api()

            # Update UI from main thread
            root.after(0, lambda: on_data_loaded(data))
        except Exception as e:
            root.after(0, lambda: on_error(str(e)))

    # Start background thread
    thread = threading.Thread(target=background_task, daemon=True)
    thread.start()

def on_data_loaded(data):
    """Called on main thread when data is loaded"""
    # Update UI
    for item in data:
        listbox.insert("end", item)

    status_label.config(text="Data loaded successfully")
    load_button.config(state="normal")

def on_error(error_message):
    """Called on main thread when error occurs"""
    messagebox.showerror("Error", f"Failed to load data: {error_message}")
    status_label.config(text="Error loading data")
    load_button.config(state="normal")
```

---

## Platform-Specific Guidelines

### Windows (WPF)

- Use **Fluent Design** principles (acrylic, reveal, depth)
- Follow **Windows UI guidelines** for control placement
- Support **high DPI** displays with proper scaling
- Use **modern controls** from Windows Community Toolkit

### Cross-Platform (.NET MAUI)

- Use **platform-specific resources** when needed
- Test on **multiple platforms** (Windows, macOS, iOS, Android)
- Follow **platform conventions** for navigation and gestures
- Use **device-specific layouts** when necessary

---

## Summary Checklist

When generating native GUI components, ensure:

- [ ] Related controls grouped logically (frames, group boxes, panels)
- [ ] Consistent naming conventions followed
- [ ] Logical tab order defined
- [ ] Keyboard shortcuts provided for common actions
- [ ] Appropriate layout manager used (Grid for forms, Stack for lists)
- [ ] Data binding used instead of manual property updates (WPF)
- [ ] Input validation implemented with clear error messages
- [ ] Dialog boxes used for confirmations and errors
- [ ] Long-running operations run on background threads
- [ ] UI updated from main thread only
- [ ] Platform-specific guidelines followed
- [ ] Responsive layout that adapts to window resizing

These best practices ensure generated GUI applications are intuitive, maintainable, and provide excellent user experience across platforms.
