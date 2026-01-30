#!/usr/bin/env python3
"""
Component Name

Brief description of what this GUI component does.

Usage:
    python gui-template.py
"""

import tkinter as tk
from tkinter import ttk
from tkinter import messagebox


class ComponentWindow:
    """Main component window class"""

    def __init__(self, root):
        """
        Initialize the component window.

        Args:
            root: The Tkinter root window
        """
        self.root = root
        self.root.title("Component Title")
        self.root.geometry("600x400")  # width x height

        # Configure grid weights for responsive layout
        self.root.columnconfigure(0, weight=1)
        self.root.rowconfigure(0, weight=1)

        # Initialize variables
        self.example_var = tk.StringVar()
        self.checkbox_var = tk.BooleanVar()

        # Create UI
        self.create_widgets()

    def create_widgets(self):
        """Create and layout all widgets"""

        # Main frame
        main_frame = ttk.Frame(self.root, padding="10")
        main_frame.grid(row=0, column=0, sticky="nsew")
        main_frame.columnconfigure(1, weight=1)

        # Header
        header_label = ttk.Label(
            main_frame,
            text="Component Title",
            font=("TkDefaultFont", 16, "bold")
        )
        header_label.grid(row=0, column=0, columnspan=2, pady=(0, 20))

        # Example input field
        ttk.Label(main_frame, text="Example Input:").grid(
            row=1, column=0, sticky="w", pady=5
        )
        self.example_entry = ttk.Entry(
            main_frame,
            textvariable=self.example_var,
            width=40
        )
        self.example_entry.grid(row=1, column=1, sticky="ew", pady=5)
        self.example_entry.focus()  # Set initial focus

        # Example checkbox
        self.example_checkbox = ttk.Checkbutton(
            main_frame,
            text="Example Option",
            variable=self.checkbox_var
        )
        self.example_checkbox.grid(
            row=2, column=0, columnspan=2, sticky="w", pady=5
        )

        # Button frame
        button_frame = ttk.Frame(main_frame)
        button_frame.grid(row=3, column=0, columnspan=2, pady=20)

        # Submit button
        self.submit_button = ttk.Button(
            button_frame,
            text="Submit",
            command=self.handle_submit
        )
        self.submit_button.pack(side="left", padx=5)

        # Cancel button
        self.cancel_button = ttk.Button(
            button_frame,
            text="Cancel",
            command=self.handle_cancel
        )
        self.cancel_button.pack(side="left", padx=5)

        # Status bar
        self.status_label = ttk.Label(
            main_frame,
            text="Ready",
            relief="sunken",
            anchor="w"
        )
        self.status_label.grid(
            row=4, column=0, columnspan=2, sticky="ew", pady=(10, 0)
        )

    def handle_submit(self):
        """Handle submit button click"""
        # Get form values
        example_value = self.example_var.get()
        checkbox_value = self.checkbox_var.get()

        # Validate
        if not example_value:
            messagebox.showerror(
                "Validation Error",
                "Example Input is required"
            )
            self.example_entry.focus()
            return

        # Process the form
        try:
            self.status_label.config(text="Processing...")
            self.root.update_idletasks()

            # Perform your logic here
            result = self.process_form(example_value, checkbox_value)

            # Show success message
            messagebox.showinfo(
                "Success",
                f"Form submitted successfully!\nResult: {result}"
            )

            self.status_label.config(text="Success")

            # Optional: Clear form
            # self.reset_form()

        except Exception as e:
            messagebox.showerror(
                "Error",
                f"An error occurred: {str(e)}"
            )
            self.status_label.config(text="Error")

    def process_form(self, example_value, checkbox_value):
        """
        Process the form data.

        Args:
            example_value: Value from example input
            checkbox_value: Value from checkbox

        Returns:
            Processing result
        """
        # Implement your business logic here
        return f"Processed: {example_value}, Checkbox: {checkbox_value}"

    def handle_cancel(self):
        """Handle cancel button click"""
        if messagebox.askyesno(
            "Confirm Cancel",
            "Are you sure you want to cancel?"
        ):
            self.reset_form()

    def reset_form(self):
        """Reset form to initial state"""
        self.example_var.set("")
        self.checkbox_var.set(False)
        self.status_label.config(text="Ready")
        self.example_entry.focus()


def main():
    """Main entry point"""
    # Create root window
    root = tk.Tk()

    # Set window icon (optional)
    # root.iconbitmap("icon.ico")

    # Create component
    app = ComponentWindow(root)

    # Set minimum window size
    root.minsize(400, 300)

    # Center window on screen
    root.update_idletasks()
    width = root.winfo_width()
    height = root.winfo_height()
    x = (root.winfo_screenwidth() // 2) - (width // 2)
    y = (root.winfo_screenheight() // 2) - (height // 2)
    root.geometry(f"{width}x{height}+{x}+{y}")

    # Start event loop
    root.mainloop()


if __name__ == "__main__":
    main()
