import React, { useState } from "react";
import dynamic from "next/dynamic";
import "suneditor/dist/css/suneditor.min.css";

const SunEditor = dynamic(
  () => import("suneditor-react").then((mod) => mod.default),
  {
    ssr: false,
  }
);

const EditorPage: React.FC = () => {
  const [content, setContent] = useState("");

  const handleEditorChange = (content: string) => {
    setContent(content);
  };

  const handleTextareaChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    setContent(e.target.value);
  };

  return (
    <div style={{ display: "flex" }}>
      <SunEditor
        height="500px"
        setContents={content}
        setOptions={{
          buttonList: [
            // ...
          ],
        }}
        onChange={handleEditorChange}
      />
      <textarea
        style={{ marginLeft: "1rem", width: "50%", height: "500px" }}
        value={content}
        onChange={handleTextareaChange}
      />
    </div>
  );
};

export default EditorPage;
