import { useState } from "react";
import {
  Editor,
  EditorState,
  convertToRaw,
  convertFromRaw,
  RichUtils,
  ContentState,
} from "draft-js";
import "draft-js/dist/Draft.css";
import { stateToHTML } from "draft-js-export-html";
import { convertFromHTML } from "draft-convert";
import Toolbar from "./Toolbar";

type Props = {
  onChange: (editorState: EditorState) => void;
  editorState: EditorState;
};

const DraftEditor = ({ onChange, editorState }: Props) => {
  const onEditorStateChange = (newEditorState: EditorState) => {
    onChange(newEditorState);
  };

  const handleKeyCommand = (command: string, editorState: EditorState) => {
    const newState = EditorState.undo(editorState);
    if (newState) {
      onChange(newState);
      return "handled";
    }
    return "not-handled";
  };

  const onInlineStyleClick = (inlineStyle: string) => {
    onChange(RichUtils.toggleInlineStyle(editorState, inlineStyle));
  };

  const onBlockTypeClick = (blockType: string) => {
    onChange(RichUtils.toggleBlockType(editorState, blockType));
  };

  return (
    <div style={{ border: "1px solid black", padding: "10px", width: "50%" }}>
      <Toolbar
        onBlockTypeClick={onBlockTypeClick}
        onStyleClick={onInlineStyleClick}
        blockType={currentBlockType}
        inlineStyle={currentBlockStyle}
      />

      <Editor
        editorState={editorState}
        onChange={onEditorStateChange}
        handleKeyCommand={handleKeyCommand}
      />
    </div>
  );
};

const draftToHtml = (content?: ContentState) => {
  if (!content) {
    return "";
  }
  const rawContent = convertToRaw(content);
  if (!rawContent || !rawContent.blocks) {
    return "";
  }
  const contentState = convertFromRaw(rawContent);
  return stateToHTML(contentState);
};

const htmlToDraft = (html: string) => {
  if (!html) {
    return ContentState.createFromText("");
  }
  const contentState = convertFromHTML(html);
  if (!contentState) {
    return ContentState.createFromText("");
  }
  return contentState;
};

const convertToHtml = (content: ContentState) => {
  return draftToHtml(content);
};

const convertFromHtml = (html: string) => {
  const contentState = htmlToDraft(html);
  return EditorState.createWithContent(contentState);
};

export { DraftEditor, convertToHtml, convertFromHtml };
