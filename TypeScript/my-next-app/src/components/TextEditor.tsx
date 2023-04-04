import { useState } from "react";
import { Editor, EditorState } from "draft-js";
import "draft-js/dist/Draft.css";
import EditorToolbar from "./EditorToolbar";

const TextEditor = () => {
  const [editorState, setEditorState] = useState(() =>
    EditorState.createEmpty()
  );

  const onChange = (editorState: EditorState) => {
    setEditorState(editorState);
  };

  return (
    <div>
      <EditorToolbar
        editorState={editorState}
        setEditorState={setEditorState}
      />
      <div style={{ border: "1px solid #ccc", minHeight: "200px" }}>
        <Editor
          editorState={editorState}
          onChange={onChange}
          placeholder="Write something"
        />
      </div>
    </div>
  );
};

export default TextEditor;
