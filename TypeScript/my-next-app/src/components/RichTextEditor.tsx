// src/components/RichTextEditor.tsx
import React, { useRef, useState } from "react";
import { EditorState, RichUtils, Modifier, ContentBlock } from "draft-js";
import { Editor } from "draft-js";
import "draft-js/dist/Draft.css";
import styles from "../../styles/RichTextEditor.module.css";
import { Map as ImmutableMap } from "immutable";
import Toolbar from "./Toolbar2";

interface RichTextEditorProps {
  editorState: EditorState;
  onChange: (editorState: EditorState) => void;
}

const styleMap = {
  "12px": {
    fontSize: "12px",
  },
  "16px": {
    fontSize: "16px",
  },
  "24px": {
    fontSize: "24px",
  },
  "32px": {
    fontSize: "32px",
  },
  Arial: {
    fontFamily: "Arial, sans-serif",
  },
  CourierNew: {
    fontFamily: '"Courier New", Courier, monospace',
  },
  Georgia: {
    fontFamily: "Georgia, serif",
  },
  TimesNewRoman: {
    fontFamily: '"Times New Roman", Times, serif',
  },
  MS_Gothic: { fontFamily: '"MS Gothic", sans-serif' },
  LINK: {
    color: "blue",
    textDecoration: "underline",
  },
};

const RichTextEditor: React.FC<RichTextEditorProps> = ({
  onChange,
  editorState,
}) => {
  const editorRef = useRef<Editor>(null);
  const [currentFontSize, setCurrentFontSize] = useState("16px");
  const [currentFontFamily, setCurrentFontFamily] = useState("Arial");

  const handleKeyCommand = (command: string, editorState: EditorState) => {
    const newState = RichUtils.handleKeyCommand(editorState, command);
    if (newState) {
      onChange(newState);
      return "handled";
    }
    return "not-handled";
  };

  const focusEditor = () => {
    editorRef.current?.focus();
  };

  const toggleStyle = (styleType: "inline" | "block", style: string) => {
    if (styleType === "inline") {
      onChange(RichUtils.toggleInlineStyle(editorState, style));
    } else {
      onChange(RichUtils.toggleBlockType(editorState, style));
    }
  };

  const applyFontSize = (fontSize: string) => {
    const newEditorState = RichUtils.toggleInlineStyle(editorState, fontSize);
    setCurrentFontSize(fontSize);
    onChange(newEditorState);
  };

  const applyFontFamily = (fontFamily: string) => {
    const newEditorState = RichUtils.toggleInlineStyle(editorState, fontFamily);
    setCurrentFontFamily(fontFamily);
    onChange(newEditorState);
  };

  const LinkSpan = (props: any) => {
    const { children, block } = props;
    const blockData = block.getData();
    const url = blockData.get("data-href");

    if (url) {
      return (
        <a href={url} style={{ color: "blue", textDecoration: "underline" }}>
          {children}
        </a>
      );
    } else {
      return <span>{children}</span>;
    }
  };

  const customBlockRendererFn = (contentBlock: ContentBlock) => {
    const type = contentBlock.getType();
    if (type === "atomic") {
      return {
        component: LinkSpan,
        editable: true,
        props: {
          block: contentBlock,
        },
      };
    }
  };

  const addLink = () => {
    const selection = editorState.getSelection();
    if (!selection.isCollapsed()) {
      const url = window.prompt("Enter the URL of the link:", "https://");
      if (url) {
        const updatedEditorState = createLink(editorState, url);
        onChange(updatedEditorState);
        setTimeout(() => editorRef.current?.focus(), 0);
      }
    } else {
      alert("Please select the text you want to turn into a link.");
    }
  };

  const createLink = (editorState: EditorState, url: string) => {
    const contentState = editorState.getCurrentContent();
    const contentStateWithEntity = contentState.createEntity(
      "LINK",
      "MUTABLE",
      { url }
    );
    const entityKey = contentStateWithEntity.getLastCreatedEntityKey();
    const newEditorState = EditorState.set(editorState, {
      currentContent: contentStateWithEntity,
    });

    const selection = editorState.getSelection();
    const newContentState = Modifier.applyEntity(
      newEditorState.getCurrentContent(),
      selection,
      entityKey
    );
    const updatedEditorState = EditorState.push(
      editorState,
      newContentState,
      "apply-entity"
    );

    return updatedEditorState;
  };

  const textAlignLeft = () => {
    changeTextAlignment("left");
  };

  const textAlignCenter = () => {
    changeTextAlignment("center");
  };

  const textAlignRight = () => {
    changeTextAlignment("right");
  };

  const changeTextAlignment = (alignment: string) => {
    const contentState = editorState.getCurrentContent();
    const selection = editorState.getSelection();
    const blockData = ImmutableMap<string, string>().set(
      "textAlign",
      alignment
    );

    const newContentState = Modifier.setBlockData(
      contentState,
      selection,
      blockData
    );
    const updatedEditorState = EditorState.push(
      editorState,
      newContentState,
      "change-block-data"
    );

    onChange(updatedEditorState);
  };

  const blockStyleFn = (contentBlock: ContentBlock) => {
    const textAlign = contentBlock.getData().get("textAlign");
    return textAlign ? `textAlign-${textAlign}` : "";
  };

  const increaseIndent = () => {
    changeIndent(true);
  };

  const decreaseIndent = () => {
    changeIndent(false);
  };

  const changeIndent = (increase: boolean) => {
    const contentState = editorState.getCurrentContent();
    const selection = editorState.getSelection();
    const newContentState = Modifier.adjustBlockDepth(
      contentState,
      selection,
      increase ? 1 : -1
    );
    const updatedEditorState = EditorState.push(
      editorState,
      newContentState,
      "adjust-depth"
    );

    onChange(updatedEditorState);
  };

  return (
    <div>
      <Toolbar
        toggleStyle={toggleStyle}
        applyFontSize={applyFontSize}
        applyFontFamily={applyFontFamily}
        addLink={addLink}
        textAlignLeft={textAlignLeft}
        textAlignCenter={textAlignCenter}
        textAlignRight={textAlignRight}
        increaseIndent={increaseIndent}
        decreaseIndent={decreaseIndent}
        currentFontSize={currentFontSize}
        currentFontFamily={currentFontFamily}
      />
      <div
        onClick={focusEditor}
        className={`public-DraftEditor-root ${styles.publicDraftEditorContent}`}
      >
        <Editor
          ref={editorRef}
          editorState={editorState}
          handleKeyCommand={handleKeyCommand}
          blockRendererFn={customBlockRendererFn}
          blockStyleFn={blockStyleFn}
          onChange={onChange}
          customStyleMap={styleMap}
        />
      </div>
    </div>
  );
};

export default RichTextEditor;
