// pages/editor.tsx
import React, { useState, useEffect } from "react";
import { Editor } from "@tiptap/react";
import StarterKit from "@tiptap/starter-kit";
import Underline from "@tiptap/extension-underline";
import TiptapEditor from "../components/Tiptap/TiptapEditor";
import SyncedTextarea from "../components/Tiptap/SyncedTextarea";
import Toolbar from "../components/Tiptap/Toolbar";

const EditorPage: React.FC = () => {
  const [content, setContent] = useState("");
  const [editor, setEditor] = useState<Editor | null>(null);

  useEffect(() => {
    if (typeof window !== "undefined") {
      const newEditor = new Editor({
        extensions: [StarterKit, Underline],
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
        <div style={{ width: "50%" }}>
          <TiptapEditor editor={editor} />
        </div>
        <SyncedTextarea
          content={content}
          onContentChange={(newContent) => {
            setContent(newContent);
            editor?.commands.setContent(newContent);
          }}
        />
      </div>
    </div>
  );
};

export default EditorPage;
