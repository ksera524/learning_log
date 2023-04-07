// components/Tiptap/Toolbar.tsx
import React from "react";
import { Editor } from "@tiptap/react";

interface ToolbarProps {
  editor: Editor | null;
}

const Toolbar: React.FC<ToolbarProps> = ({ editor }) => {
  if (!editor) {
    return null;
  }

  const fontFamilies = [
    { label: "Inter", value: "Inter" },
    { label: "Comic Sans", value: "Comic Sans MS, Comic Sans" },
    { label: "serif", value: "serif" },
    { label: "monospace", value: "monospace" },
    { label: "cursive", value: "cursive" },
    { label: "MS ゴシック", value: "MS Gothic, MS ゴシック" },
    { label: "Times New Roman", value: "Times New Roman, Times, serif" },
  ];

  const fontSizes = [
    { label: "8pt", value: "8" },
    { label: "10pt", value: "10" },
    { label: "12pt", value: "12" },
    { label: "14pt", value: "14" },
    { label: "16pt", value: "16" },
    { label: "18pt", value: "18" },
    { label: "24pt", value: "24" },
    { label: "36pt", value: "36" },
    { label: "48pt", value: "48" },
  ];

  return (
    <div style={{ display: "flex", marginBottom: "1rem" }}>
      <button onClick={() => editor.chain().focus().toggleBold().run()}>
        Bold
      </button>
      <button
        onClick={() => editor.chain().focus().toggleItalic().run()}
        style={{ marginLeft: "0.5rem" }}
      >
        Italic
      </button>
      <button
        onClick={() => editor.chain().focus().toggleUnderline().run()}
        style={{ marginLeft: "0.5rem" }}
      >
        Underline
      </button>
      <select
        style={{ marginLeft: "0.5rem" }}
        defaultValue=""
        onChange={(event) => {
          const fontFamily = event.target.value;
          editor.chain().focus().setFontFamily(fontFamily).run();
        }}
      >
        <option value="" disabled>
          Font Family
        </option>
        {fontFamilies.map((fontFamily, index) => (
          <option key={index} value={fontFamily.value}>
            {fontFamily.label}
          </option>
        ))}
      </select>
      <select
        style={{ marginLeft: "0.5rem" }}
        defaultValue=""
        onChange={(event) => {
          const fontSize = event.target.value;
          editor.chain().focus().setFontSize(`${fontSize}pt`).run();
        }}
      >
        <option value="" disabled>
          Font Size
        </option>
        {fontSizes.map((fontSize, index) => (
          <option key={index} value={fontSize.value}>
            {fontSize.label}
          </option>
        ))}
      </select>
    </div>
  );
};

export default Toolbar;
