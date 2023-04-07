// components/TiptapEditor.tsx
import React from "react";
import { Editor, EditorContent } from "@tiptap/react";

interface TiptapEditorProps {
  editor: Editor | null;
}

const TiptapEditor: React.FC<TiptapEditorProps> = ({ editor }) => {
  if (!editor) {
    return <div>Loading...</div>;
  }

  return <EditorContent editor={editor} />;
};

export default TiptapEditor;
