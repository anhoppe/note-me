import { useEffect, useState } from 'react'
import 'bootstrap/dist/css/bootstrap.min.css';

import './App.css'
import { Note } from "./Note"
import NoteForm from './NoteForm';


function App() {
  const [notes, setNotes] = useState<Note[]>([]);
  const [selectedIndex, setSelectedIndex] = useState(-1);
  // const apiUrl = "https://note-me-backend-991989948061.us-central1.run.app/notes";
  const apiUrl = "http://127.0.0.1:8080/notes";

  useEffect(() => {
    const getNotes = async () => {

      try {
            const data = await fetchNotes();
            
            const notesData = data.map(noteData => {
              const note = new Note(noteData.id)

              note.text = noteData.text;
              note.title = noteData.title;
              note.createdAt = new Date(noteData.createdAt); // Convert string to Date

              return note;
          });

          setNotes(notesData);
        } catch (err) {
            // setError((err as Error).message);
        }
    };

    getNotes();
  }, []); // Empty dependency array ensures this runs only once

  const fetchNotes = async (): Promise<Note[]> => {
    const response = await fetch(apiUrl);

    if (!response.ok) {
        throw new Error("Failed to fetch notes");
    }

    return response.json();
  };

  const handleAddNote = async () => {
    const newNote = new Note(BigInt(notes.length));
    newNote.text = "foo";
    newNote.title = "bar";

    let success = await createNote(newNote);

    if (success)
    {    
      setNotes([...notes, newNote]);
    }  
  };

  const createNote = async (note: Note): Promise<boolean> => {
    try {
      const payload = JSON.stringify(note);
      const response = await fetch(apiUrl, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: payload, // Convert the note object to JSON
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

  const updateNote = async (note: Note) => {
    try {
      const payload = JSON.stringify(note);
      const url = apiUrl + '/' + note.id.toString();
      const response = await fetch(url, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: payload,
      });
  
      if (!response.ok) {
        // Handle HTTP errors (e.g., 400, 500)
        const errorText = await response.text(); // Get error message from server
        alert(`HTTP error ${response.status}: ${errorText} endpoint:${apiUrl}`);
        return false;
      }
  
      console.log('Note updated successfully');  
    } catch (error) {
      console.error('Error updating note:', error);
      alert("Error updating note:" + error);
      return false;
    }
  }

  const handleSelectText = (index: number) => {
    setSelectedIndex(index);
  }

  const changeNoteText = (value: string) => {
    const newNotes = [...notes];
    let note = newNotes[selectedIndex];
    note.text = value;
    setNotes(newNotes);
  };

  const changeNoteTitle = (value: string) => {
    const newNotes = [...notes];
    newNotes[selectedIndex].title = value;
    setNotes(newNotes);
  }

  const sendSelectedNoteToBackend = () => {
    const newNotes = [...notes];
    let note = newNotes[selectedIndex];
    updateNote(note);
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
            updateNoteTextFunc={changeNoteText}
            updateNoteTitleFunc={changeNoteTitle}
            sendNoteToBackend={sendSelectedNoteToBackend}
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