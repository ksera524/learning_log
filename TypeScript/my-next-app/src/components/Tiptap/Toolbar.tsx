// components/Tiptap/Toolbar.tsx
import React, { useCallback } from "react";
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

  const setLink = useCallback(() => {
    const previousUrl = editor.getAttributes("link").href;
    const url = window.prompt("URL", previousUrl);

    // cancelled
    if (url === null) {
      return;
    }

    // empty
    if (url === "") {
      editor.chain().focus().extendMarkRange("link").unsetLink().run();

      return;
    }

    // update link
    editor.chain().focus().extendMarkRange("link").setLink({ href: url }).run();
  }, [editor]);

  const handleTextAlign = (align: string) => {
    editor.chain().focus().setTextAlign(align).run();
  };

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
      <input
        type="color"
        style={{ marginLeft: "0.5rem" }}
        onChange={(event) => {
          const color = event.target.value;
          editor.chain().focus().setColor(color).run();
        }}
      />
      <button
        onClick={setLink}
        className={editor.isActive("link") ? "is-active" : ""}
      >
        setLink
      </button>
      <button
        onClick={() => editor.chain().focus().unsetLink().run()}
        disabled={!editor.isActive("link")}
      >
        unsetLink
      </button>
      <button
        onClick={() => handleTextAlign("left")}
        style={{ marginLeft: "0.5rem" }}
      >
        Left
      </button>
      <button
        onClick={() => handleTextAlign("center")}
        style={{ marginLeft: "0.5rem" }}
      >
        Center
      </button>
      <button
        onClick={() => editor.chain().focus().setTextAlign("right").run()}
        style={{ marginLeft: "0.5rem" }}
      >
        Right
      </button>
    </div>
  );
};

export default Toolbar;
