// src/components/RichTextEditor.tsx
import React, { useRef, useState } from "react";
import { EditorState, RichUtils } from "draft-js";
import { Editor } from "draft-js";
import "draft-js/dist/Draft.css";

interface RichTextEditorProps {
  editorState: EditorState;
  onChange: (editorState: EditorState) => void;
}

const styleMap = {
  "12px": {
    fontSize: "12px",
  },
  "16px": {
    fontSize: "16px",
  },
  "24px": {
    fontSize: "24px",
  },
  "32px": {
    fontSize: "32px",
  },
};

const RichTextEditor: React.FC<RichTextEditorProps> = ({
  onChange,
  editorState,
}) => {
  const editorRef = useRef<Editor>(null);
  const [currentFontSize, setCurrentFontSize] = useState("16px");

  const handleKeyCommand = (command: string, editorState: EditorState) => {
    const newState = RichUtils.handleKeyCommand(editorState, command);
    if (newState) {
      onChange(newState);
      return "handled";
    }
    return "not-handled";
  };

  const focusEditor = () => {
    editorRef.current?.focus();
  };

  const toggleStyle = (styleType: "inline" | "block", style: string) => {
    if (styleType === "inline") {
      onChange(RichUtils.toggleInlineStyle(editorState, style));
    } else {
      onChange(RichUtils.toggleBlockType(editorState, style));
    }
  };

  const applyFontSize = (fontSize: string) => {
    const newEditorState = RichUtils.toggleInlineStyle(editorState, fontSize);
    setCurrentFontSize(fontSize);
    onChange(newEditorState);
  };

  return (
    <div>
      <div>
        <button onClick={() => toggleStyle("inline", "BOLD")}>Bold</button>
        <button onClick={() => toggleStyle("inline", "ITALIC")}>Italic</button>
        <button onClick={() => toggleStyle("inline", "UNDERLINE")}>
          Underline
        </button>
        <button onClick={() => toggleStyle("block", "unordered-list-item")}>
          Bullet List
        </button>
        <button onClick={() => toggleStyle("block", "ordered-list-item")}>
          Numbered List
        </button>
        <select
          value={currentFontSize}
          onChange={(e) => applyFontSize(e.target.value)}
        >
          <option value="12px">12px</option>
          <option value="16px">16px</option>
          <option value="24px">24px</option>
          <option value="32px">32px</option>
        </select>
      </div>
      <div onClick={focusEditor}>
        <Editor
          ref={editorRef}
          editorState={editorState}
          handleKeyCommand={handleKeyCommand}
          onChange={onChange}
          customStyleMap={styleMap}
        />
      </div>
    </div>
  );
};

export default RichTextEditor;
