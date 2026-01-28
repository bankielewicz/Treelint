"""
Exceptions for headless mode answer resolution (STORY-098).
"""


class HeadlessResolutionError(Exception):
    """
    Raised when a prompt cannot be resolved in headless mode.

    AC#3: Fail-on-Unanswered Mode
    - When fail_on_unanswered: true and no matching answer exists
    - Error message includes prompt text for debugging
    """

    def __init__(self, prompt_text: str, message: str = None):
        self.prompt_text = prompt_text
        if message is None:
            message = f"Headless mode: No answer configured for prompt '{prompt_text}'"
        super().__init__(message)


class ConfigurationError(Exception):
    """
    Raised when ci-answers.yaml configuration is invalid.

    AC#5: Answer Validation on Load
    - Invalid YAML syntax
    - Missing required fields
    - Invalid field values
    """

    def __init__(self, message: str, line_number: int = None):
        self.line_number = line_number
        if line_number:
            message = f"{message} (line {line_number})"
        super().__init__(message)
