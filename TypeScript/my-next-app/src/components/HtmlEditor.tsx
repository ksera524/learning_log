import { useState } from "react";
import { EditorState, ContentState } from "draft-js";
import { convertFromHTML } from "draft-convert";
import { convertToHTML } from "draft-convert";
import { Toolbar } from "./Toolbar";

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
  const rawContent = convertToHTML(content);
  if (!rawContent) {
    return "";
  }
  return rawContent;
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

export { HtmlEditor, convertToHtml, convertFromHtml };
