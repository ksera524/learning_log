import { useState } from "react";
import { EditorState, ContentState } from "draft-js";
import { convertFromHTML } from "draft-convert";
import { convertToHTML } from "draft-convert";

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

  const onInsertTemplateClick = () => {
    const contentState = ContentState.createFromText("<h1>Hello World</h1>");
    const templateHtml = convertToHTML(contentState)
      .replace(/&lt;/g, "<")
      .replace(/&gt;/g, ">")
      .replace("<p>", "")
      .replace("</p>", "");
    const newHtml = `${html}${templateHtml}`;
    onChange(newHtml);
    const newContentState = htmlToDraft(newHtml);
    setEditorState(EditorState.createWithContent(newContentState));
  };

  return (
    <div style={{ border: "1px solid black", padding: "10px", width: "50%" }}>
      <Toolbar onInsertTemplateClick={onInsertTemplateClick} />
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

type ToolbarProps = {
  onInsertTemplateClick: () => void;
};

const Toolbar = ({ onInsertTemplateClick }: ToolbarProps) => {
  return (
    <div style={{ marginBottom: "10px" }}>
      <button type="button" onClick={onInsertTemplateClick}>
        Insert Template
      </button>
    </div>
  );
};

export { HtmlEditor, convertToHtml, convertFromHtml };
