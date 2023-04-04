// components/RichTextEditor.tsx
import React, { useState } from "react";
import { Editor, EditorState, RichUtils } from "draft-js";
import "draft-js/dist/Draft.css";

type RichTextEditorProps = {
  onChange: (editorState: EditorState) => void;
  editorState: EditorState;
};

const RichTextEditor: React.FC<RichTextEditorProps> = ({
  onChange,
  editorState,
}) => {
  const handleKeyCommand = (
    command: string,
    editorState: EditorState
  ): "handled" | "not-handled" => {
    const newState = RichUtils.handleKeyCommand(editorState, command);

    if (newState) {
      onChange(newState);
      return "handled";
    }

    return "not-handled";
  };

  return (
    <div className="editor-container">
      <Editor
        editorState={editorState}
        handleKeyCommand={handleKeyCommand}
        onChange={onChange}
      />
      <style jsx>{`
        .editor-container {
          border: 1px solid #ccc;
          padding: 1rem;
          min-height: 200px;
        }
      `}</style>
    </div>
  );
};

export default RichTextEditor;
