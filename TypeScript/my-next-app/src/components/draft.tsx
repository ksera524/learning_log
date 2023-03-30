import { useState } from "react";
import {
  Editor,
  EditorState,
  RichUtils,
  convertToRaw,
  convertFromRaw,
  ContentState,
} from "draft-js";
import "draft-js/dist/Draft.css";
import { stateToHTML } from "draft-js-export-html";
import { convertFromHTML } from "draft-convert";

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

type HtmlEditorProps = {
  onChange: (html: string) => void;
  html: string;
};

const HtmlEditor = ({ onChange, html }: HtmlEditorProps) => {
  const [editorState, setEditorState] = useState(EditorState.createEmpty());
  const onHtmlEditorStateChange = (
    event: React.ChangeEvent<HTMLTextAreaElement>
  ) => {
    const html = event.target.value;
    const contentState = htmlToDraft(html);
    onChange(html);
    setEditorState(EditorState.createWithContent(contentState));
  };

  return (
    <div style={{ border: "1px solid black", padding: "10px", width: "50%" }}>
      <textarea value={html} onChange={onHtmlEditorStateChange} />
    </div>
  );
};

const draftToHtml = (content?: ContentState) => {
  if (!content) {
    return "";
  }
  const rawContent = convertToRaw(content);
  if (!rawContent || !rawContent.blocks) {
    return "";
  }
  const contentState = convertFromRaw(rawContent);
  return stateToHTML(contentState);
};

const htmlToDraft = (html: string) => {
  if (!html) {
    return ContentState.createFromText("");
  }
  const contentState = convertFromHTML(html);
  if (!contentState) {
    return ContentState.createFromText("");
  }
  return contentState;
};

const convertToHtml = (content: ContentState) => {
  return draftToHtml(content);
};

const convertFromHtml = (html: string) => {
  const contentState = htmlToDraft(html);
  return EditorState.createWithContent(contentState);
};

type ToolbarProps = {
  onBoldClick: () => void;
  onUnderlineClick: () => void;
  onItalicClick: () => void;
};

const Toolbar = ({
  onBoldClick,
  onUnderlineClick,
  onItalicClick,
}: ToolbarProps) => {
  return (
    <div style={{ marginBottom: "10px" }}>
      <button type="button" onClick={onBoldClick}>
        Bold
      </button>
      <button type="button" onClick={onUnderlineClick}>
        Underline
      </button>
      <button type="button" onClick={onItalicClick}>
        Italic
      </button>
    </div>
  );
};

export { DraftEditor, HtmlEditor, convertToHtml, convertFromHtml };
