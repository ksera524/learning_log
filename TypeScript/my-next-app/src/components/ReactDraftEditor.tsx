import { EditorState } from "draft-js";
import "react-draft-wysiwyg/dist/react-draft-wysiwyg.css";

import dynamic from "next/dynamic";

const Editor = dynamic(
  () => import("react-draft-wysiwyg").then((mod) => mod.Editor),
  { ssr: false }
);

type Props = {
  onChange: (editorState: EditorState) => void;
  editorState: EditorState;
};

const DraftEditor = ({ onChange, editorState }: Props) => {
  const onEditorStateChange = (newEditorState: EditorState) => {
    onChange(newEditorState);
  };

  return (
    <div style={{ border: "1px solid black", padding: "10px", width: "50%" }}>
      <Editor
        editorState={editorState}
        toolbar={{}}
        onEditorStateChange={onEditorStateChange}
      />
    </div>
  );
};

export { DraftEditor };
