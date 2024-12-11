import { useState } from 'react'
import 'bootstrap/dist/css/bootstrap.min.css';

import './App.css'
import { Note } from "./Note"
import NoteForm from './NoteForm';


function App() {
  const [notes, setNotes] = useState<Note[]>([]);
  const [selectedIndex, setSelectedIndex] = useState(-1);

  const handleAddNote = () => {
    const newNote = new Note();
    newNote.text = "foo";
    newNote.title = "bar";
    setNotes([...notes, newNote]);
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