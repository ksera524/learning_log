// pages/editor.tsx
import React, { useState, useEffect } from "react";
import { EditorState } from "draft-js";
import RichTextEditor from "../components/RichTextEditor";

const EditorPage: React.FC = () => {
  const [editorState, setEditorState] = useState(EditorState.createEmpty());
  const [editorEnable, setEditorEnable] = useState(false);

  useEffect(() => {
    setEditorEnable(true);
  }, []);

  return (
    <div className="container">
      <h1>Rich Text Editor</h1>
      {editorEnable && (
        <RichTextEditor onChange={setEditorState} editorState={editorState} />
      )}
      <style jsx>{`
        .container {
          max-width: 800px;
          margin: 0 auto;
          padding: 2rem;
        }
      `}</style>
    </div>
  );
};

export default EditorPage;
