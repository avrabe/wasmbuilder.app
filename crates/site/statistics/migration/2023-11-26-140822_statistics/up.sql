-- Create the table with components as INTEGER PRIMARY KEY
CREATE TABLE IF NOT EXISTS statistics (components INTEGER PRIMARY KEY);

-- Insert a row with 0 as the value for components
INSERT INTO statistics VALUES (0);

-- Create a trigger that will update the value of components whenever a new row is inserted
CREATE TRIGGER IF NOT EXISTS update_components AFTER INSERT ON statistics
BEGIN
  -- Delete the new row
  DELETE FROM statistics WHERE components = NEW.components;
  -- Increment the value of components in the existing row by one
  UPDATE statistics SET components = components + 1;
END;