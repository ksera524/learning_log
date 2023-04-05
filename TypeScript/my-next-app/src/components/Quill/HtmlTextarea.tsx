import React from "react";

type HtmlTextareaProps = {
  content: string;
  setContent: (content: string) => void;
};

const HtmlTextarea: React.FC<HtmlTextareaProps> = ({ content, setContent }) => {
  return (
    <textarea
      style={{ width: "100%", height: "100%" }}
      value={content}
      onChange={(e) => {
        setContent(e.target.value);
      }}
    />
  );
};

export default HtmlTextarea;
