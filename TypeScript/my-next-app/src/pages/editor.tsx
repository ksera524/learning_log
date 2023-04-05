import React, { useState } from "react";
import Editor from "../components/Quill/Editor";
import HtmlTextarea from "../components/Quill/HtmlTextarea";

const EditorPage: React.FC = () => {
  const [content, setContent] = useState("");

  return (
    <div style={{ display: "flex", height: "100vh" }}>
      <div style={{ flex: 1 }}>
        <Editor content={content} setContent={setContent} />
      </div>
      <div style={{ flex: 1 }}>
        <HtmlTextarea content={content} setContent={setContent} />
      </div>
    </div>
  );
};

export default EditorPage;
