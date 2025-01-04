import { Note } from './Note';
import 'bootstrap/dist/css/bootstrap.min.css';

interface NoteProps {
    note: Note;
    updateNoteTextFunc: (value: string) => void;
    updateNoteTitleFunc: (value: string) => void;
    sendNoteToBackend: () => void;
}

function NoteForm(props: NoteProps) {
    const {note, updateNoteTextFunc, updateNoteTitleFunc, sendNoteToBackend} = props;

    if (!note) {
        return <div>Add or select a note</div>;
    }
    return (
    <>
        <div className="col">
            <div className="row-1">
                <textarea 
                    className="form-control"
                    value={note.title} 
                    onChange={(e) => updateNoteTitleFunc(e.target.value)} 
                    onBlur={sendNoteToBackend}
                    />
            </div>
            <div className="row-11">
                <textarea 
                    className="form-control"
                    value={note.text} 
                    onChange={(e) => updateNoteTextFunc(e.target.value)} 
                    onBlur={sendNoteToBackend}
                    />
            </div>
            <div className="row-1">
                <textarea
                    className="form-control"
                    value={note.createdAt.toLocaleTimeString()}
                />
            </div>
        </div>
    </>
    );
}

export default NoteForm;
