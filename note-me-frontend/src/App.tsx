import { useState } from 'react'
import 'bootstrap/dist/css/bootstrap.min.css';

import './App.css'
import { Note } from "./Note"
import NoteForm from './NoteForm';


function App() {
  const [notes, setNotes] = useState<Note[]>([]);
  const [selectedIndex, setSelectedIndex] = useState(-1);
  const apiUrl = "http://127.0.0.1:3000/notes";

  const handleAddNote = async () => {
    const newNote = new Note();
    newNote.text = "foo";
    newNote.title = "bar";

    let success = await handleSaveNote(newNote);

    if (success)
    {    
      setNotes([...notes, newNote]);
    }  
  };

  const handleSaveNote = async (note: Note): Promise<boolean> => {
    try {
      const response = await fetch(apiUrl, {
        method: 'PUT', // Correct method is PUT as specified
        headers: {
          'Content-Type': 'application/json', // Very important!
        },
        body: JSON.stringify(note), // Convert the note object to JSON
      });

      if (!response.ok) {
        // Handle HTTP errors (e.g., 400, 500)
        const errorText = await response.text(); // Get error message from server
        alert(`HTTP error ${response.status}: ${errorText} endpoint:${apiUrl}`);
        return false;
      }

      console.log('Note saved successfully');
      // Optionally, update the UI or show a success message
    } catch (error) {
      console.error('Error saving note:', error);
      alert("Error saving note:" + error);
      return false;
      // Handle network errors or other exceptions
    }

    return true;
  };

  const handleSelectText = (index: number) => {
    setSelectedIndex(index);
  }

  const handleNoteTextChange = (value: string) => {
    const newNotes = [...notes];
    newNotes[selectedIndex].text = value;
    setNotes(newNotes);
  };

  const handleNoteTitleChange = (value: string) => {
    const newNotes = [...notes];
    newNotes[selectedIndex].title = value;
    setNotes(newNotes);
  }

  return (
    <div className="container-fluid">
      <div className="row">

        <div className="col-md-4">
          <button onClick={handleAddNote}>Add Note</button>
          {notes.map((note, index) => (
            <textarea
              key={index}
              value={note.title}
              onClick={() => handleSelectText(index)}
              className="form-control"
            />
          ))}
        </div>

        <div className="col-md-8">
          <NoteForm 
            note={notes[selectedIndex]} 
            updateNodeTextFunc={handleNoteTextChange}
            updateNodeTitleFunc={handleNoteTitleChange}
            />
        </div>
      </div>
    </div>
  )
}

export default App

          /* Replace with a real rich text editor component */
          /* <textarea 
            placeholder="Rich Text Editor" 
            className="form-control" 
            value={editedText} 
            onChange={(e) => handleNoteChange(e.target.value)} />
        </div> */