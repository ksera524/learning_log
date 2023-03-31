import { useEffect, useState } from "react";
import { EditorState } from "draft-js";
import { DraftEditor } from "@/components/ReactDraftEditor";
import { HtmlEditor } from "@/components/HtmlEditor";
import { convertToHtml } from "@/components/HtmlEditor";
import { convertFromHtml } from "@/components/HtmlEditor";

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