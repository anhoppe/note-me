import { Note } from './Note';
import 'bootstrap/dist/css/bootstrap.min.css';

interface NoteProps {
    note: Note;
    updateNodeTextFunc: (value: string) => void;
    updateNodeTitleFunc: (value: string) => void;
}

function NoteForm(props: NoteProps) {
    const {note, updateNodeTextFunc, updateNodeTitleFunc} = props;

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
                    onChange={(e) => updateNodeTitleFunc(e.target.value)} />
            </div>
            <div className="row-11">
                <textarea 
                    className="form-control"
                    value={note.text} 
                    onChange={(e) => updateNodeTextFunc(e.target.value)} />
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
