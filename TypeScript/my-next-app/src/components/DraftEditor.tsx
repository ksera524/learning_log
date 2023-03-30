import { Editor, EditorState, RichUtils } from "draft-js";
import "draft-js/dist/Draft.css";
import { Toolbar } from "./Toolbar";

type Props = {
  onChange: (editorState: EditorState) => void;
  editorState: EditorState;
};

const DraftEditor = ({ onChange, editorState }: Props) => {
  const onEditorStateChange = (newEditorState: EditorState) => {
    onChange(newEditorState);
  };

  const handleKeyCommand = (command: string, editorState: EditorState) => {
    const newState = RichUtils.handleKeyCommand(editorState, command);
    if (newState) {
      onChange(newState);
      return "handled";
    }
    return "not-handled";
  };

  const onBoldClick = () => {
    onChange(RichUtils.toggleInlineStyle(editorState, "BOLD"));
  };

  const onUnderlineClick = () => {
    onChange(RichUtils.toggleInlineStyle(editorState, "UNDERLINE"));
  };

  const onItalicClick = () => {
    onChange(RichUtils.toggleInlineStyle(editorState, "ITALIC"));
  };

  return (
    <div style={{ border: "1px solid black", padding: "10px", width: "50%" }}>
      <Toolbar
        onBoldClick={onBoldClick}
        onUnderlineClick={onUnderlineClick}
        onItalicClick={onItalicClick}
      />
      <Editor
        editorState={editorState}
        onChange={onEditorStateChange}
        handleKeyCommand={handleKeyCommand}
      />
    </div>
  );
};

export { DraftEditor };
