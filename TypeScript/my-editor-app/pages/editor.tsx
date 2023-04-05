import { EditorState } from "draft-js";
import dynamic from "next/dynamic";
import "react-draft-wysiwyg/dist/react-draft-wysiwyg.css";
import { useState } from "react";

const Editor = dynamic(
  () => import("react-draft-wysiwyg").then((mod) => mod.Editor),
  { ssr: false }
);

const MyEditor = () => {
  const [editorState, setEditorState] = useState(() =>
    EditorState.createEmpty()
  );

  const onEditorStateChange = (newEditorState: EditorState) => {
    setEditorState(newEditorState);
  };

  return (
    <div>
      <Editor
        editorState={editorState}
        onEditorStateChange={onEditorStateChange}
        toolbar={{
          options: [
            "inline",
            "blockType",
            "fontSize",
            "fontFamily",
            "list",
            "textAlign",
            "colorPicker",
            "link",
            "embedded",
            "emoji",
            "remove",
            "history",
          ],
          inline: {
            options: [
              "bold",
              "italic",
              "underline",
              "strikethrough",
              "monospace",
              "superscript",
              "subscript",
            ],
          },
        }}
      />
    </div>
  );
};

export default MyEditor;
