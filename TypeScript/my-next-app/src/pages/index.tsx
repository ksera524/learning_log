// pages/editor.tsx
import React, { useState, useEffect } from "react";
import { Editor } from "@tiptap/react";
import Document from "@tiptap/extension-document";
import FontFamily from "@tiptap/extension-font-family";
import Paragraph from "@tiptap/extension-paragraph";
import Text from "@tiptap/extension-text";
import TextStyle from "@tiptap/extension-text-style";
import Bold from "@tiptap/extension-bold";
import Italic from "@tiptap/extension-italic";
import Underline from "@tiptap/extension-underline";
import FontSize from "@tobiasafischer/tiptap-extension-font-size";
import TiptapEditor from "../components/Tiptap/TiptapEditor";
import { Color } from "@tiptap/extension-color";
import SyncedTextarea from "../components/Tiptap/SyncedTextarea";
import Toolbar from "../components/Tiptap/Toolbar";

const EditorPage: React.FC = () => {
  const [content, setContent] = useState("");
  const [editor, setEditor] = useState<Editor | null>(null);

  useEffect(() => {
    if (typeof window !== "undefined") {
      const newEditor = new Editor({
        extensions: [
          Document,
          Paragraph,
          Text,
          TextStyle,
          FontFamily,
          FontSize,
          Bold,
          Italic,
          Underline,
          Color,
        ],
        onUpdate: ({ editor }) => {
          setContent(editor.getHTML());
        },
      });
      setEditor(newEditor);

      // Clean up on component unmount
      return () => {
        newEditor.destroy();
      };
    }
  }, []);

  return (
    <div>
      <Toolbar editor={editor} />
      <div style={{ display: "flex" }}>
        <div style={{ flex: 1 }}>
          <TiptapEditor editor={editor} />
        </div>
        <SyncedTextarea
          content={content}
          onContentChange={(newContent) => {
            setContent(newContent);
            editor?.commands.setContent(newContent);
          }}
        />
        <div
          style={{ flex: 1 }}
          dangerouslySetInnerHTML={{ __html: content }}
        />
      </div>
    </div>
  );
};

export default EditorPage;
