import React from "react";

interface SyncedTextareaProps {
  content: string;
  onContentChange: (content: string) => void;
}

const SyncedTextarea: React.FC<SyncedTextareaProps> = ({
  content,
  onContentChange,
}) => {
  const handleTextareaChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    onContentChange(e.target.value);
  };

  return (
    <textarea
      style={{ marginLeft: "1rem", width: "50%", height: "500px" }}
      value={content}
      onChange={handleTextareaChange}
    />
  );
};

export default SyncedTextarea;
