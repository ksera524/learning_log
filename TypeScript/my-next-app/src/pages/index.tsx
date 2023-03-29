import { useEffect, useState } from "react";
import { EditorState } from "draft-js";
import {
  DraftEditor,
  HtmlEditor,
  convertToHtml,
  convertFromHtml,
} from "../components/draft";

export default function Home() {
  const [editorEnable, setEditorEnable] = useState(false);
  const [editorState, setEditorState] = useState(EditorState.createEmpty());
  const [html, setHtml] = useState("");

  useEffect(() => {
    setEditorEnable(true);
  }, []);

  const onEditorStateChange = (newEditorState: EditorState) => {
    setEditorState(newEditorState);
    const html = convertToHtml(newEditorState.getCurrentContent());
    setHtml(html);
  };

  const onHtmlChange = (newHtml: string) => {
    setHtml(newHtml);
    const editorState = convertFromHtml(newHtml);
    setEditorState(editorState);
  };

  return (
    <div style={{ display: "flex" }}>
      {editorEnable && (
        <DraftEditor editorState={editorState} onChange={onEditorStateChange} />
      )}
      <HtmlEditor html={html} onChange={onHtmlChange} />
    </div>
  );
}
