import { useState } from "react";
import Editor from "@/components/lexical";

const EditorPage = () => {
  const [content, setContent] = useState("");

  const handleChange = (value: string) => {
    setContent(value);
  };

  return (
    <div>
      <Editor />
    </div>
  );
};

export default EditorPage;
