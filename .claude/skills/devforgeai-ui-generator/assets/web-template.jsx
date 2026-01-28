import React, { useState, useEffect } from 'react';
import PropTypes from 'prop-types';

/**
 * ComponentName - Brief description of what this component does
 *
 * @param {Object} props - Component props
 * @param {Function} props.onAction - Callback function when action occurs
 * @param {Boolean} props.isLoading - Loading state indicator
 * @param {Object} props.data - Data to display in component
 */
function ComponentName({ onAction, isLoading, data }) {
  // State management
  const [localState, setLocalState] = useState({
    // Initialize component state here
  });

  // Side effects
  useEffect(() => {
    // Component mount/update side effects
    console.log('Component mounted or updated');

    // Cleanup function
    return () => {
      console.log('Component will unmount');
    };
  }, [/* dependencies */]);

  // Event handlers
  const handleAction = (event) => {
    event.preventDefault();

    // Handle the action
    if (onAction) {
      onAction(localState);
    }
  };

  const handleInputChange = (event) => {
    const { name, value } = event.target;
    setLocalState(prev => ({
      ...prev,
      [name]: value
    }));
  };

  // Conditional rendering
  if (isLoading) {
    return (
      <div className="loading" role="status" aria-live="polite">
        <p>Loading...</p>
      </div>
    );
  }

  // Main render
  return (
    <div className="component-container">
      <header className="component-header">
        <h2>Component Title</h2>
      </header>

      <main className="component-main">
        {/* Component content goes here */}
        <form onSubmit={handleAction}>
          <div className="form-group">
            <label htmlFor="exampleInput">
              Example Input
            </label>
            <input
              type="text"
              id="exampleInput"
              name="exampleInput"
              value={localState.exampleInput || ''}
              onChange={handleInputChange}
              placeholder="Enter value"
              required
            />
          </div>

          <button type="submit" disabled={isLoading}>
            Submit
          </button>
        </form>
      </main>

      {data && (
        <footer className="component-footer">
          <p>Data: {JSON.stringify(data)}</p>
        </footer>
      )}
    </div>
  );
}

// PropTypes validation
ComponentName.propTypes = {
  onAction: PropTypes.func,
  isLoading: PropTypes.bool,
  data: PropTypes.object,
};

// Default props
ComponentName.defaultProps = {
  onAction: null,
  isLoading: false,
  data: null,
};

export default ComponentName;
