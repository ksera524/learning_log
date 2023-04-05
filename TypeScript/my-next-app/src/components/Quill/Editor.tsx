import React, { useState } from "react";
import DynamicReactQuill from "./DynamicReactQuill";
import "react-quill/dist/quill.snow.css";

type EditorProps = {
  content: string;
  setContent: (content: string) => void;
};

const Editor: React.FC<EditorProps> = ({ content, setContent }) => {
  return <DynamicReactQuill content={content} setContent={setContent} />;
};

export default Editor;
