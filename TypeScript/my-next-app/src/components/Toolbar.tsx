import React from "react";

interface ToolbarProps {
  onStyleClick: (style: string) => void;
  onBlockTypeClick: (blockType: string) => void;
  inlineStyle: DraftInlineStyle;
  blockType: string;
}

const Toolbar = ({
  onBoldClick,
  onUnderlineClick,
  onItalicClick,
  onStyleClick,
}: ToolbarProps) => {
  const handleStyleClick = (event: React.MouseEvent<HTMLButtonElement>) => {
    const style = event.currentTarget.getAttribute("data-style");
    onStyleClick(style || "");
  };

  return (
    <div style={{ marginBottom: "10px" }}>
      <button type="button" onClick={onBoldClick}>
        Bold
      </button>
      <button type="button" onClick={onUnderlineClick}>
        Underline
      </button>
      <button type="button" onClick={onItalicClick}>
        Italic
      </button>
      <button type="button" onClick={handleStyleClick} data-style="red">
        Red
      </button>
      <button type="button" onClick={handleStyleClick} data-style="blue">
        Blue
      </button>
    </div>
  );
};

export default Toolbar;
