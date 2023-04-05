import { EditorState, RichUtils } from "draft-js";
import styles from "./EditorToolbar.module.css";

const INLINE_STYLES = [
  { label: "Bold", style: "BOLD" },
  { label: "Italic", style: "ITALIC" },
  { label: "Underline", style: "UNDERLINE" },
];

const BLOCK_TYPES = [
  { label: "H1", style: "header-one" },
  { label: "H2", style: "header-two" },
  { label: "H3", style: "header-three" },
  { label: "H4", style: "header-four" },
  { label: "H5", style: "header-five" },
  { label: "H6", style: "header-six" },
  { label: "Blockquote", style: "blockquote" },
  { label: "UL", style: "unordered-list-item" },
  { label: "OL", style: "ordered-list-item" },
  { label: "Code Block", style: "code-block" },
];

interface EditorToolbarProps {
  editorState: EditorState;
  setEditorState: (editorState: EditorState) => void;
}

const EditorToolbar = (props: EditorToolbarProps) => {
  const { editorState, setEditorState } = props;

  const toggleBlockType = (blockType: string) => {
    setEditorState(RichUtils.toggleBlockType(editorState, blockType));
  };

  const toggleInlineStyle = (inlineStyle: string) => {
    setEditorState(RichUtils.toggleInlineStyle(editorState, inlineStyle));
  };

  const currentStyle = editorState.getCurrentInlineStyle();
  const selection = editorState.getSelection();
  const blockType = editorState
    .getCurrentContent()
    .getBlockForKey(selection.getStartKey())
    .getType();

  return (
    <div className={styles.RichEditorControls}>
      {INLINE_STYLES.map((type) => (
        <StyleButton
          key={type.label}
          active={currentStyle.has(type.style)}
          label={type.label}
          onToggle={toggleInlineStyle}
          style={type.style}
        />
      ))}
      {BLOCK_TYPES.map((type) => (
        <StyleButton
          key={type.label}
          active={type.style === blockType}
          label={type.label}
          onToggle={toggleBlockType}
          style={type.style}
        />
      ))}
    </div>
  );
};

interface StyleButtonProps {
  active: boolean;
  label: string;
  onToggle: (style: string) => void;
  style: string;
}

const StyleButton = (props: StyleButtonProps) => {
  const { active, label, onToggle, style } = props;

  const handleToggle = (e: React.MouseEvent<HTMLSpanElement>) => {
    e.preventDefault();
    onToggle(style);
  };

  let className = styles.RichEditorStyleButton;
  if (active) {
    className += " " + styles.RichEditorActiveButton;
  }

  return (
    <span className={className} onMouseDown={handleToggle}>
      {label}
    </span>
  );
};

export default EditorToolbar;
